use super::{
    assembly::Assembly,
    function::Function,
    instruction::Instruction,
    model::{EnumerableModel, LitteralModel},
    parameter::Parameter,
    prompt::Prompt, return_type_coercion::ReturnTypeCoercion
};

struct AssemblyReader {
    lines: Vec<String>,
    cursor: usize,
}

impl AssemblyReader {
    pub fn new(source: &String) -> Self {
        AssemblyReader {
            lines: source.lines().map(str::to_string).collect(),
            cursor: 0,
        }
    }

    pub fn reached_end(&self) -> bool {
        self.cursor >= self.lines.len()
    }

    pub fn next_instruction(&mut self) -> Result<(String, Vec<String>), String> {
        let line = self.peek_instruction()?;
        self.cursor += 1;

        Ok(line)
    }

    pub fn peek_instruction(&self) -> Result<(String, Vec<String>), String> {
        if self.reached_end() {
            return Err("Tried to get line after end of assembly was reached".to_string());
        }

        let line: String = self.lines.get(self.cursor).unwrap().clone();
        let tokens: Vec<String> = line.split(" ").map(str::to_string).collect();

        if tokens.is_empty() {
            return Err("Line is empty".to_string());
        }

        Ok((tokens.first().unwrap().to_string(), tokens[1..].into()))
    }

    pub fn peek_next_instruction(&mut self) -> Result<(String, Vec<String>), String> {
        self.next();
        let next_instruction = self.peek_instruction();
        self.previous();

        next_instruction
    }

    pub fn next_line(&mut self) -> Result<String, String> {
        if self.reached_end() {
            return Err("Tried to get line after end of assembly was reached".to_string());
        }

        let line: String = self.lines.get(self.cursor).unwrap().clone();
        self.cursor += 1;

        Ok(line)
    }

    pub fn next(&mut self) {
        self.cursor += 1;
    }

    pub fn previous(&mut self) {
        self.cursor -= 1;
    }

    pub fn expect(&mut self, instruction: &str) -> Result<Vec<String>, String> {
        let (next_instruction, parameters) = self.next_instruction()?;

        if next_instruction.as_str() == instruction {
            Ok(parameters)
        }
        else {
            Err(format!("Expected {}, found {}", instruction, next_instruction))
        }
    }
}

pub fn load_assembly(source: &String) -> Result<Assembly, String> {
    let mut assembly: Assembly = Assembly::new();
    let mut reader: AssemblyReader = AssemblyReader::new(source);

    while !reader.reached_end() {
        match reader.next_instruction() {
            Ok((instruction, parameters)) => {
                match instruction.as_str() {
                    "MODULE" => {
                        assembly.name = parameters.get(0).unwrap().clone();
                    },
                    "MODEL" => {
                        let name: String = parameters.get(0).unwrap().clone();
                        let mut text: String = String::new();

                        reader.expect("START")?;
                        loop {
                            let (line_instruction, _) = reader.peek_instruction()?;
                            if line_instruction == "END" {
                                break;
                            }

                            text += reader.next_line()?.as_str();
                        }
                        reader.next();

                        assembly.models.insert(
                            name.clone(),
                            LitteralModel { name, text }.into(),
                        );
                    },
                    "ENUM_MODEL" => {
                        let name: String = parameters.get(0).unwrap().clone();
                        let possible_models: Vec<String> = parameters[1..].into();

                        assembly.models.insert(
                            name.clone(),
                            EnumerableModel { name, possible_models }.into(),
                        );
                    },
                    "PROMPT" => {
                        let name: String = parameters.get(0).unwrap().clone();
                        let mut text: String = String::new();

                        let arguments: Vec<Parameter> = reader
                            .expect("ARGUMENTS")?[0..]
                            .iter()
                            .map(|argument| Parameter { name: argument.clone() })
                            .collect();
                        let returns: String = reader.expect("RETURNS")?.get(0).unwrap().clone();

                        reader.expect("START")?;
                        loop {
                            let (line_instruction, _) = reader.peek_instruction()?;
                            if line_instruction == "END" {
                                break;
                            }

                            text += reader.next_line()?.as_str();
                        }
                        reader.next();

                        assembly.prompts.insert(
                            name.clone(),
                            Prompt {
                                name,
                                parameters: arguments,
                                return_type: returns,
                                text,
                            }
                        );
                    },
                    "FUNCTION" => {
                        let name: String = parameters.get(0).unwrap().clone();
                        let mut instructions: Vec<Instruction> = Vec::new();

                        let arguments: Vec<Parameter> = reader
                            .expect("ARGUMENTS")?[0..]
                            .iter()
                            .map(|argument| Parameter { name: argument.clone() })
                            .collect();
                        let returns: String = reader.expect("RETURNS")?.get(0).unwrap().clone();

                        reader.expect("START")?;
                        loop {
                            let (line_instruction, line_parameters) = reader.peek_instruction()?;
                            match line_instruction.as_str() {
                                "END" => {
                                    break;
                                },
                                "ASSIGN" => {
                                    instructions.push(
                                        Instruction::Assign(
                                            line_parameters.get(0).unwrap().to_string(),
                                            line_parameters.get(1).unwrap().to_string(),
                                        )
                                    );
                                },
                                "INVOKE" => {
                                    let (next_instruction, coerce_arguments) = reader.peek_next_instruction()?;

                                    let coerce_return: ReturnTypeCoercion = if next_instruction == "COERCE_RETURN" {
                                        reader.next();
                                        match coerce_arguments.get(0) {
                                            Some(return_type) => ReturnTypeCoercion::Embbedded(return_type.clone()),
                                            None => ReturnTypeCoercion::NotCoerced,
                                        }
                                    }
                                    else {
                                        ReturnTypeCoercion::NotCoerced
                                    };

                                    instructions.push(
                                        Instruction::Invoke(
                                            line_parameters.get(0).unwrap().clone(),
                                            line_parameters[1..].into(),
                                            coerce_return,
                                        )
                                    );
                                },
                                "RETURN" => {
                                    instructions.push(
                                        Instruction::Return(
                                            line_parameters.get(0).unwrap().clone(),
                                        )
                                    )
                                },
                                _ => {},
                            }
                            reader.next();
                        }
                        reader.next();

                        if let Some(index) = instructions.iter()
                            .rposition(|i| matches!(i, Instruction::Invoke(_, _, _)))
                        {
                            if let Some(Instruction::Invoke(_, _, coerce_return_type)) = instructions.get_mut(index) {
                                if let ReturnTypeCoercion::Embbedded(_) = coerce_return_type {
                                    return Err(
                                        format!(
                                            "Last task invocation of function {} has a coerced return type. This is illegal, remove type coercion on the last invocation.",
                                            name
                                        )
                                    );
                                }
                                else {
                                    *coerce_return_type = ReturnTypeCoercion::FromCaller;
                                }
                            }
                        }

                        assembly.functions.insert(
                            name.clone(),
                            Function {
                                name,
                                parameters: arguments,
                                return_type: returns,
                                instructions,
                            }
                        );
                    },
                    _ => {},
                }
            },
            Err(_) => {
                continue;
            }
        }
    }

    Ok(assembly)
}
