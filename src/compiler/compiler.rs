use super::command::Command;
use super::token::Token;

pub struct Compiler {
    tokens: Vec<Token>,
    commands: Vec<Command>,
    file_source: String,
}

impl Compiler {
    pub fn new(file_source: String) -> Compiler {
        let mut return_compiler: Compiler = Compiler {
            tokens: Vec::new(),
            commands: Vec::new(),
            file_source,
        };
        return_compiler.prepare_source();
        return_compiler
    }

    fn prepare_source(&mut self) {
        self.file_source.retain(|c| c != '\r');
        self.file_source.push('\n');
    }

    fn tokenise(&mut self) -> Result<(), String> {
        let mut read_buffer: String = String::new();

        for ch in self.file_source.chars() {
            if ch.is_whitespace() {
                match read_buffer.as_str() {
                    "LDA" => self.tokens.push(Token::Load),
                    "STA" => self.tokens.push(Token::Store),
                    "ADD" => self.tokens.push(Token::Add),
                    "SUB" => self.tokens.push(Token::Subtract),
                    "INP" => self.tokens.push(Token::Input),
                    "OUT" => self.tokens.push(Token::Output),
                    "HLT" => self.tokens.push(Token::End),
                    "BRA" => self.tokens.push(Token::BranchAll),
                    "BRZ" => self.tokens.push(Token::BranchZero),
                    "BRP" => self.tokens.push(Token::BranchZeroPos),
                    "DAT" => self.tokens.push(Token::DataStore),

                    _ => {
                        if !read_buffer.is_empty() {
                            match read_buffer.parse::<i16>() {
                                Ok(value) => self.tokens.push(Token::IntLiteral(value)),
                                Err(_e) => {
                                    return Err(format!(
                                        "[Error] Unknown identifier {}",
                                        read_buffer
                                    ));
                                }
                            }
                        }
                    }
                }
                if ch == '\n' {
                    self.tokens.push(Token::Endline)
                }
                read_buffer.clear();
            } else {
                read_buffer.push(ch);
            }
        }

        Ok(())
    }

    fn create_cmd_structure(&mut self) -> Result<(), String> {
        let mut token_buffer: [Token; 2] = [Token::NoToken; 2];
        for token in &self.tokens {
            if token == &Token::Endline {
                match token_buffer[0] {
                    Token::NoToken => {}
                    Token::End => self.commands.push(Command::HLT),
                    Token::Input => self.commands.push(Command::INP),
                    Token::Output => self.commands.push(Command::OUT),
                    Token::IntLiteral(_) => {
                        return Err("Integer literal at start of line".to_string())
                    }
                    _ => match token_buffer[1] {
                        Token::IntLiteral(data) => {
                            if token_buffer[0] != Token::DataStore && (0 >= data || data >= 99) {
                                return Err(format!(
                                    "Location provided: {} not in address range",
                                    data
                                ));
                            }
                            match token_buffer[0] {
                                Token::Add => self.commands.push(Command::ADD(data as u8)),
                                Token::Subtract => self.commands.push(Command::SUB(data as u8)),
                                Token::Load => self.commands.push(Command::LDA(data as u8)),
                                Token::Store => self.commands.push(Command::STA(data as u8)),
                                Token::BranchAll => self.commands.push(Command::BRA(data as u8)),
                                Token::BranchZero => self.commands.push(Command::BRZ(data as u8)),
                                Token::BranchZeroPos => {
                                    self.commands.push(Command::BRP(data as u8))
                                }
                                Token::DataStore => self.commands.push(Command::DAT(data)),
                                _ => {}
                            }
                        }
                        _ => {
                            return Err(
                                "Data expectant token provided without int literal".to_string()
                            )
                        }
                    },
                }

                token_buffer = [Token::NoToken, Token::NoToken];
            } else {
                'no_full: {
                    for i in 0..=1 {
                        if token_buffer[i] == Token::NoToken {
                            token_buffer[i] = *token;
                            break 'no_full;
                        }
                    }

                    return Err("Too many back-to-back tokens".to_string());
                }
            }
        }

        Ok(())
    }

    fn create_memory_array(&self) -> Result<[i16; 100], String> {
        let mut memory: [i16; 100] = [0; 100];
        let mut program_counter = 0;
        if self.commands.len() > 100 {
            return Err(format!("Program too long: length {}", self.commands.len()));
        }
        for cmd in &self.commands {
            memory[program_counter] = match cmd.get_byte_data() {
                Ok(data) => data,
                Err(_) => return Err(format!("Collosal fuck-up on my end my bad g")),
            };
            program_counter += 1;
        }
        Ok(memory)
    }

    pub fn compile(&mut self) -> Result<[i16; 100], String> {
        match self.tokenise() {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        match self.create_cmd_structure() {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        self.create_memory_array()
    }
}
