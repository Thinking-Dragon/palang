use std::collections::HashMap;

use crate::{assembly::{function::Function, instruction::Instruction}, llm::model_settings::ModelSettings};

use super::virtual_machine::VirtualMachine;

pub async fn run_function<'a>(
    function_info: &'a Function,
    parameters: &Vec<String>,
    model_settings: &ModelSettings,
    vm: &'a mut VirtualMachine,
) -> Result<String, String> {
    let mut runner: FunctionRunner = FunctionRunner {
        model_settings,
        vm,
        function_info,
        variables: HashMap::new(),
        invocation_registry: None,
        program_counter: 0,
    };

    load_parameters_into_variables(&mut runner, function_info, parameters);

    loop {
        match runner.step().await {
            StepResult::Ok => continue,
            StepResult::Return(value) => return Ok(value),
            StepResult::Err(e) => return Err(e),
        }
    }
}

fn load_parameters_into_variables(
    runner: &mut FunctionRunner,
    function_info: &Function,
    parameters: &Vec<String>
) {
    for (
        _, (parameter, value)
    ) in function_info.parameters.iter().zip(parameters.iter()).enumerate() {
        runner.variables.insert(parameter.name.to_string(), value.to_string());
    }
}

pub struct FunctionRunner<'a> {
    model_settings: &'a ModelSettings,
    vm: &'a mut VirtualMachine,
    function_info: &'a Function,
    variables: HashMap<String, String>,
    invocation_registry: Option<String>,
    program_counter: usize,
}

enum StepResult {
    Ok,
    Return(String),
    Err(String),
}

impl<'a> FunctionRunner<'a> {
    async fn step(&mut self) -> StepResult {
        match self.function_info.instructions.get(self.program_counter) {
            Some(instruction) => {
                match instruction {
                    Instruction::Assign(to, from) => {
                        if from == "@invocation_registry" {
                            match &self.invocation_registry {
                                Some(value) => {
                                    self.variables.insert(to.clone(), value.clone());
                                },
                                None => {
                                    return StepResult::Err("Tried to assign value from empty invocation registry".to_string());
                                },
                            }
                        }
                        else {
                            match self.variables.get(from) {
                                Some(value) => {
                                    self.variables.insert(to.clone(), value.clone());
                                },
                                None => {
                                    return StepResult::Err(format!("Variable {} not found", from));
                                },
                            }
                        }

                        self.program_counter += 1;
                    },
                    Instruction::Invoke(task, arguments) => {
                        let argument_values: Vec<String> = arguments
                            .iter()
                            .map(|argument_name| self.variables.get(argument_name).unwrap().clone())
                            .collect();

                        self.invocation_registry = match self.vm.execute(
                            task,
                            &argument_values,
                            &self.model_settings,
                        ).await.await {
                            Ok(value) => Some(value.clone()),
                            Err(_) => None,
                        };

                        self.program_counter += 1;
                    },
                    Instruction::Return(to_return) => {
                        match self.variables.get(to_return) {
                            Some(value) => {
                                return StepResult::Return(value.clone());
                            },
                            None => {
                                return StepResult::Return(to_return.clone());
                            },
                        }
                    },
                }
            },
            None => {
                return StepResult::Err("Tried to execute instruction outside of function bounds.".to_string())
            }
        }
        StepResult::Ok
    }
}
