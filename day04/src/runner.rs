use std::collections::{HashMap, VecDeque};

use crate::statement::{LoopType as StatementLoopType, Statement};

pub struct Runner {
    variables: HashMap<u8, u8>,
    loop_state: VecDeque<LoopCounter>,
    should_exit: bool,
}

impl Runner {
    pub fn execute_statement(&mut self, statement: Statement, statement_index: &mut usize) {
        match statement {
            Statement::Set(id, value) => {
                self.variables.insert(id, value);
            }
            Statement::Operation(id, op, value) => {
                // If variable exists
                if let Some(var_value) = self.variables.get_mut(&id) {
                    *var_value = op.operate(*var_value, value);
                } else {
                    panic!("Tried to operate on variable '{id}' that does not exist.");
                }
            }
            Statement::Print(id) => {
                if let Some(var_value) = self.variables.get(&id) {
                    println!("{}", var_value);
                } else {
                    panic!("Tried to print variable '{id}' that does not exist.");
                }
            }
            Statement::Speak(id) => {
                if let Some(var_value) = self.variables.get(&id) {
                    println!("{}", crate::speak_map::convert_u8(*var_value));
                } else {
                    panic!("Tried to print variable '{id}' that does not exist.");
                }
            }
            Statement::Loop(loop_type) => {
                if let StatementLoopType::Number(n) = loop_type {
                    self.loop_state.push_back(LoopCounter {
                        start_statement: *statement_index,
                        loop_type: LoopType::Number(n),
                    });
                } else if let StatementLoopType::Variable(id) = loop_type {
                    self.loop_state.push_back(LoopCounter {
                        start_statement: *statement_index,
                        loop_type: LoopType::Variable(id),
                    })
                } else if let StatementLoopType::End = loop_type {
                    let current_loop = self
                        .loop_state
                        .back_mut()
                        .expect("There's been an error with loop counting. :(");

                    if let LoopType::Number(n) = current_loop.loop_type {
                        if n == 0 {
                            return;
                        }

                        current_loop.loop_type = LoopType::Number(n - 1);
                    } else if let LoopType::Variable(id) = current_loop.loop_type {
                        if let Some(var_value) = self.variables.get_mut(&id) {
                            if *var_value == 0 {
                                return;
                            }

                            *var_value -= 1;
                        } else {
                            panic!("Tried to loop variable '{id}' that does not exist.");
                        }
                    }

                    *statement_index = current_loop.start_statement.clone();
                }
            }
            Statement::Stop => {
                self.should_exit = true;
            }
        }
    }

    pub fn execute_statements(&mut self, statements: Vec<Statement>) {
        if statements.is_empty() {
            return;
        }

        let mut current_statement = 0;

        while !self.should_exit {
            // println!("Loop State: {:?}", self.loop_state);

            if current_statement >= statements.len() {
                panic!("Statement pointer exceeded amount of statements.");
            }

            self.execute_statement(statements[current_statement], &mut current_statement);

            current_statement += 1;
        }
    }
}

impl Default for Runner {
    fn default() -> Self {
        Runner {
            variables: HashMap::new(),
            loop_state: VecDeque::new(),
            should_exit: false,
        }
    }
}

#[derive(Debug)]
struct LoopCounter {
    start_statement: usize,
    loop_type: LoopType,
}

// statement::LoopType without End
#[derive(Debug)]
enum LoopType {
    Number(u8),
    Variable(u8),
}

pub fn create_runner() -> Runner {
    Runner::default()
}
