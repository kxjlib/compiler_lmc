use std::collections::VecDeque;
use std::mem::discriminant;

use super::command::Command;
use super::token::Token;

pub struct Compiler {}

impl Compiler {
    pub fn new(file_source: String) -> Compiler {
        Compiler {}
    }

    fn prepare_source(file_source: String) -> String {
        let mut source: String = file_source.clone();
        source.retain(|c: char| c != '\r' && c != '\t');
        source.push('\n');
        source
    }

    // Convert a string (data_source) into a vector of words
    fn acquire_words(file_source: String) -> Vec<String> {
        let mut word_vector: Vec<String> = Vec::new();

        let mut last_index = 0;
        for (index, separator) in file_source.match_indices(|ch: char| ch == '\n' || ch == ' ') {
            if last_index != index {
                word_vector.push(file_source[last_index..index].to_string());
            }
            if separator != " " {
                word_vector.push(separator.to_string());
            }
            last_index = index + 1;
        }
        if last_index < file_source.len() {
            word_vector.push(file_source[last_index..].to_string());
        }

        return word_vector;
    }

    fn tokenise(word_vector: Vec<String>) -> Result<VecDeque<Token>, String> {
        let mut token_queue: VecDeque<Token> = VecDeque::new();

        for word in word_vector {
            match word.as_str() {
                "LDA" => token_queue.push_back(Token::Load),
                "STA" => token_queue.push_back(Token::Store),
                "ADD" => token_queue.push_back(Token::Add),
                "SUB" => token_queue.push_back(Token::Subtract),
                "INP" => token_queue.push_back(Token::Input),
                "OUT" => token_queue.push_back(Token::Output),
                "HLT" => token_queue.push_back(Token::End),
                "BRA" => token_queue.push_back(Token::BranchAll),
                "BRZ" => token_queue.push_back(Token::BranchZero),
                "BRP" => token_queue.push_back(Token::BranchZeroPos),
                "DAT" => token_queue.push_back(Token::DataStore),
                "\n" => token_queue.push_back(Token::Endline),
                _ => {
                    let is_numeric: bool = word.chars().all(char::is_numeric);
                    let is_string: bool = word.chars().all(char::is_alphabetic);

                    if is_numeric {
                        match word.parse::<i16>() {
                            Ok(value) => token_queue.push_back(Token::IntLiteral(value)),
                            Err(_e) => {
                                return Err(format!("[Error] Number Conversion {}", word));
                            }
                        }
                    }
                }
            }
        }

        Ok(token_queue)
    }

    fn create_token_groups_validate(
        tokens_to_group: VecDeque<Token>,
    ) -> Result<Vec<VecDeque<Token>>, String> {
        let mut token_buffer: VecDeque<Token> = VecDeque::new();
        let mut tokens_remaining: VecDeque<Token> = tokens_to_group.clone();

        let mut valid_next_tokens: Vec<Token> = Token::Endline.next_valid(true);

        let mut token_groups: Vec<VecDeque<Token>> = Vec::new();

        while let Some(token) = tokens_remaining.pop_front() {
            // Ensure that the token which is next in the queue is a valid token for the flow of the program
            if valid_next_tokens.contains(&token)
                || (valid_next_tokens.contains(&Token::IntLiteralNoData)
                    && discriminant(&token) == discriminant(&Token::IntLiteral(0)))
            {
                if token == Token::Endline {
                    if token_buffer.len() != 0 {
                        println!("{:?}", token_buffer);
                        token_groups.push(token_buffer.clone());
                    }
                    token_buffer.clear();
                } else {
                    token_buffer.push_back(token);
                }
                valid_next_tokens = token.next_valid(token_buffer.is_empty());
            } else {
                return Err(format!("[Error] Invalid Token: {:?}", token));
            }
        }

        Ok(token_groups)
    }

    fn create_cmd_structure(tokens_to_compile: VecDeque<Token>) -> Result<Vec<Command>, String> {
        let mut command_buffer: Vec<Command> = Vec::new();

        let token_groups: Vec<VecDeque<Token>> =
            match Self::create_token_groups_validate(tokens_to_compile) {
                Ok(tokens) => tokens,
                Err(e) => return Err(e),
            };

        for group in token_groups {
            let mut comp_group = group.clone();
            // We can guarantee that the group has at least one element in it so unwrap should be alright
            let first_token: Token = match comp_group.pop_front() {
                Some(token) => token,
                None => unreachable!(),
            };
            match first_token {
                Token::Load
                | Token::Store
                | Token::Add
                | Token::Subtract
                | Token::BranchAll
                | Token::BranchZero
                | Token::BranchZeroPos => {
                    let location: u8;

                    match comp_group.pop_front().unwrap() {
                        Token::IntLiteral(value) => {
                            if 0 > value || value > 99 {
                                return Err(format!(
                                    "Location provided: {} not in address range",
                                    value
                                ));
                            }
                            location = value as u8;
                            command_buffer
                                .push(Command::from_token_location_literal(first_token, location));
                        }
                        _ => unreachable!(),
                    }
                }
                Token::Output => command_buffer.push(Command::OUT),
                Token::Input => command_buffer.push(Command::INP),
                Token::End => command_buffer.push(Command::HLT),
                Token::DataStore => match comp_group.pop_front() {
                    Some(value_token) => {
                        if let Token::IntLiteral(value) = value_token {
                            command_buffer.push(Command::DAT(value))
                        }
                    }
                    None => unimplemented!(),
                },
                _ => unreachable!(),
            }
        }

        println!("{:?}", command_buffer);
        Ok(command_buffer)
    }

    fn create_memory_array(command_vec: Vec<Command>) -> Result<[i16; 100], String> {
        let mut memory: [i16; 100] = [0; 100];
        let mut program_counter: usize = 0;
        if command_vec.len() > 100 {
            return Err(format!("Program too long: length {}", command_vec.len()));
        }
        for cmd in command_vec {
            memory[program_counter] = match cmd.get_byte_data() {
                Ok(data) => data,
                Err(_) => return Err(format!("Collosal fuck-up on my end my bad g")),
            };
            program_counter += 1;
        }
        Ok(memory)
    }

    pub fn compile(file_source: String) -> Result<[i16; 100], String> {
        let file_source: String = Self::prepare_source(file_source);
        let word_vector: Vec<String> = Self::acquire_words(file_source);

        let tokens_queue: VecDeque<Token> = match Self::tokenise(word_vector) {
            Ok(tokens) => tokens,
            Err(e) => return Err(e),
        };

        let command_vector = match Self::create_cmd_structure(tokens_queue) {
            Ok(commands) => commands,
            Err(e) => return Err(e),
        };

        Self::create_memory_array(command_vector)
    }
}
