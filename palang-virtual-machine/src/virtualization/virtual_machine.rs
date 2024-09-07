use std::{future::Future, pin::Pin};

use crate::{
    assembly::{
        assemblies_cache::AssembliesCache,
        assembly::Assembly,
        function::Function,
        prompt::Prompt,
        task::Task
    },
    llm::{llm::LargeLanguageModel, model_settings::ModelSettings}
};

use super::function_runner::run_function;

pub struct VirtualMachine {
    assemblies: AssembliesCache,
    llm: LargeLanguageModel,
}

impl VirtualMachine {
    pub fn new(llm: &LargeLanguageModel) -> Self {
        VirtualMachine {
            assemblies: AssembliesCache::new(),
            llm: llm.clone(),
        }
    }

    pub fn load_assembly(&mut self, assembly: &Assembly) {
        self.assemblies.load(assembly);
    }

    pub async fn execute<'a>(
        &'a mut self,
        task: &'a String,
        parameters: &'a Vec<String>,
        settings: &'a ModelSettings,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + 'a>> {
        Box::pin(async move {
            match self.assemblies.get_task(task) {
                Some(task) => {
                    match task {
                        Task::Prompt(prompt) => {
                            return self.execute_prompt(&prompt, parameters, settings).await;
                        },
                        Task::Function(function) => {
                            return self.execute_function(&function, parameters, settings).await;
                        },
                    }
                },
                None => {
                    return Err(format!("{} not found", task));
                }
            }
        })
    }

    async fn execute_prompt(
        &mut self,
        prompt: &Prompt,
        parameters: &Vec<String>,
        settings: &ModelSettings,
    ) -> Result<String, String> {
        let system: String = "
            You will reply with the wanted response only and nothing else.
            You will not add any personal remark.
            If you do not know the answer, you will say: 'unknown' and nothing else.
            You will only end your response with a dot if your response is a sentence.
            If your response is a name or a thing, you will not end it with a dot.
        ".to_string();
        let mut instructions: String = prompt.text.clone();

        for (_, (parameter, value)) in prompt.parameters.iter().zip(parameters.iter()).enumerate() {
            instructions = instructions.replace(
                &format!("@{{{}}}", parameter.name),
                &format!("{{parameter \"{}\": {}}}", parameter.name, value)
            );
        }

        instructions += "\n--- Parameter formats ---\n";
        for (_, (parameter, value)) in prompt.parameters.iter().zip(parameters.iter()).enumerate() {
            instructions += &format!("Parameter \"{}\" is formatted as follows: {}\n", parameter.name, value);
        }

        let return_type_model: String = self.assemblies.get_model(&prompt.return_type).unwrap().text.clone();
        instructions += &format!("Your response will be formatted as follows: {}", return_type_model);

        self.llm.invoke(&system, &instructions, &settings).await
    }

    async fn execute_function(
        &mut self,
        function: &Function,
        parameters: &Vec<String>,
        model_settings: &ModelSettings,
    ) -> Result<String, String> {
        run_function(function, parameters, model_settings, self).await
    }
}
