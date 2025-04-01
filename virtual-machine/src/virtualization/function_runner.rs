use std::collections::HashMap;

use palang_core::profile::Profile;

use crate::assembly::{function::Function, instruction::Instruction, return_type_coercion::ReturnTypeCoercion};

use super::virtual_machine::VirtualMachine;

pub async fn run_function<'a>(
    function_info: &'a Function,
    parameters: &Vec<String>,
    return_type_coercion: &ReturnTypeCoercion,
    profile: &Profile,
    vm: &'a mut VirtualMachine,
) -> Result<String, String> {
    let mut runner: FunctionRunner = FunctionRunner {
        profile,
        vm,
        function_info,
        return_type_coercion,
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
    for (_, (parameter, value))
        in function_info.parameters.iter().zip(parameters.iter()).enumerate()
    {
        runner.variables.insert(parameter.name.to_string(), value.to_string());
    }
}

pub struct FunctionRunner<'a> {
    profile: &'a Profile,
    vm: &'a mut VirtualMachine,
    function_info: &'a Function,
    return_type_coercion: &'a ReturnTypeCoercion,
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
                    Instruction::Invoke(task, arguments, return_type_coercion) => {
                        let argument_values: Vec<String> = arguments
                            .iter()
                            .map(|argument_name| self.variables.get(argument_name).unwrap().clone())
                            .collect();

                        let return_type_coercion = if let ReturnTypeCoercion::FromCaller = return_type_coercion {
                            self.return_type_coercion
                        }
                        else {
                            return_type_coercion
                        };

                        self.invocation_registry = match self.vm.execute(
                            task,
                            &argument_values,
                            return_type_coercion,
                            &self.profile,
                        ).await.await {
                            Ok(value) => Some(value.clone()),
                            Err(e) => {
                                eprintln!("Error while executing prompt ({})", e);
                                None
                            },
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
