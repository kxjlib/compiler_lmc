use std::{env, fmt::Display};

enum CLArgHandlerState {
    InputFile,
    OutputFile,
    Default,
}

impl Display for CLArgHandlerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enum_name = match self {
            CLArgHandlerState::InputFile => "-i",
            CLArgHandlerState::OutputFile => "-o",
            CLArgHandlerState::Default => "DEFAULTSTATE",
        };

        write!(f, "{}", enum_name)
    }
}

pub struct CLArgHandler {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    handler_state: CLArgHandlerState,
}

impl CLArgHandler {
    pub fn new() -> CLArgHandler {
        CLArgHandler {
            input_file: None,
            output_file: None,
            handler_state: CLArgHandlerState::Default,
        }
    }

    pub fn parse_arguments(&mut self) -> Result<(), String> {
        let mut cl_args: env::Args = env::args();
        // Pop first element from args (executable name)
        cl_args.next();

        while cl_args.len() != 0 {
            let arg: String = cl_args.next().unwrap();

            match arg.as_str() {
                "-i" | "-o" => {
                    // Ensure that a previous state has already been handled
                    match self.handler_state {
                        CLArgHandlerState::Default => {}
                        CLArgHandlerState::InputFile | CLArgHandlerState::OutputFile => {
                            return Err(format!(
                                "[ERROR] {} flag used without following argument",
                                self.handler_state
                            ))
                        }
                    }

                    // Change Handler State in accordance to flag provided
                    match arg.as_str() {
                        "-i" => self.handler_state = CLArgHandlerState::InputFile,
                        "-o" => self.handler_state = CLArgHandlerState::OutputFile,
                        _ => {}
                    }
                }
                _ => match self.handler_state {
                    CLArgHandlerState::InputFile | CLArgHandlerState::OutputFile => {
                        match self.handler_state {
                            CLArgHandlerState::InputFile => self.input_file = Some(arg),
                            CLArgHandlerState::OutputFile => self.output_file = Some(arg),
                            _ => {}
                        };
                        self.handler_state = CLArgHandlerState::Default;
                    }
                    CLArgHandlerState::Default => {}
                },
            }
        }
        match self.handler_state {
            CLArgHandlerState::Default => {}
            CLArgHandlerState::InputFile | CLArgHandlerState::OutputFile => {
                return Err(format!(
                    "[ERROR] {} flag used without following argument",
                    self.handler_state
                ))
            }
        }

        Ok(())
    }

}