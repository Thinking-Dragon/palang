use std::{future::Future, pin::Pin};

use palang_core::profile::Profile;

use crate::{
    assembly::{
        assemblies_cache::AssembliesCache, assembly::Assembly, function::Function, model::{LitteralModel, Model}, prompt::Prompt, return_type_coercion::ReturnTypeCoercion, task::Task
    },
    llm::llm::LargeLanguageModel
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
        coerced_type: &'a ReturnTypeCoercion,
        profile: &'a Profile,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + 'a>> {
        Box::pin(async move {
            match self.assemblies.get_task(task) {
                Some(task) => {
                    match task {
                        Task::Prompt(prompt) => {
                            return self.execute_prompt(&prompt, parameters, coerced_type, profile).await;
                        },
                        Task::Function(function) => {
                            return self.execute_function(&function, parameters, coerced_type, profile).await;
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
        coerced_type: &ReturnTypeCoercion,
        profile: &Profile,
    ) -> Result<String, String> {
        let return_type: Model = self.assemblies.get_model(&prompt.return_type)
            .ok_or(format!("Prompt \"{}\" has inexistent return type: \"{}\"", prompt.name, prompt.return_type))?;

        let return_type: LitteralModel = match coerced_type {
            ReturnTypeCoercion::Embbedded(coerced_type) => self.coerced_type(prompt, &return_type, coerced_type),
            ReturnTypeCoercion::NotCoerced => {
                match return_type {
                    Model::Litteral(model) => {
                        Ok(model)
                    },
                    Model::Enumerable(model) => {
                        let possible_types: String = model.possible_models.join(", ");
                        
                        Err(
                            format!(
                                "Prompt \"{}\" has enumerable return type \"{}\". You must coerce it to one of the following litteral types: {}",
                                prompt.name,
                                prompt.return_type,
                                possible_types,
                            )
                        )
                    },
                }
            }
            ReturnTypeCoercion::FromCaller => Err(
                format!("Prompt {} was expecting its return type to be coerced by caller ; no value was provided.", prompt.name)
            ),
        }?;

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
                &format!("${{{}}}", parameter.name),
                &format!("{{parameter \"{}\": {}}}", parameter.name, value)
            );
        }

        instructions += "\n--- Parameter formats ---\n";
        for (_, (parameter, value)) in prompt.parameters.iter().zip(parameters.iter()).enumerate() {
            instructions += &format!("Parameter \"{}\" is formatted as follows: {}\n", parameter.name, value);
        }

        let return_type_model: String = return_type.text.clone();
        instructions += &format!("Your response will be formatted as follows: {}", return_type_model);

        self.llm.invoke(&system, &instructions, &profile).await
    }

    async fn execute_function(
        &mut self,
        function: &Function,
        parameters: &Vec<String>,
        return_type_coercion: &ReturnTypeCoercion,
        profile: &Profile,
    ) -> Result<String, String> {
        run_function(function, parameters, return_type_coercion, profile, self).await
    }

    fn coerced_type(
        &self,
        prompt: &Prompt,
        return_type: &Model,
        coerced_type: &String,
    ) -> Result<LitteralModel, String> {
        match return_type {
            Model::Litteral(model) => {
                if &model.name != coerced_type {
                    Err(
                        format!(
                            "Prompt \"{}\" has litteral return type \"{}\". Litteral return types cannot be coerced.",
                            prompt.name,
                            prompt.return_type,
                        )
                    )
                }
                else {
                    Ok(model.clone())
                }
            }
            Model::Enumerable(model) => {
                if model.possible_models.contains(&coerced_type) {
                    match self.assemblies.get_model(&coerced_type) {
                        Some(Model::Litteral(coerced_type)) => {
                            Ok(coerced_type)
                        },
                        Some(Model::Enumerable(coerced_type)) => {
                            Err(
                                format!(
                                    "You tried to coerce return type \"{}\" to enumerable model \"{}\". You can only coerce to litteral models.",
                                    prompt.return_type,
                                    coerced_type.name,
                                )
                            )
                        },
                        None => {
                            Err(
                                format!(
                                    "You tried to coerce return type \"{}\" to inexistant type \"{}\"",
                                    prompt.return_type,
                                    coerced_type,
                                )
                            )
                        },
                    }
                }
                else {
                    let possible_types: String = model.possible_models.join(", ");

                    Err(
                        format!(
                            "You tried to coerce return type \"{}\" to \"{}\" which is not allowed. Allowed types are: {}",
                            prompt.return_type,
                            coerced_type,
                            possible_types,
                        )
                    )
                }
            }
        }
    }
}
