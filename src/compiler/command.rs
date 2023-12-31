use std::num::ParseIntError;

use super::token::Token;

#[derive(Debug)]
pub(in crate::compiler) enum Command {
    LDA(u8),
    STA(u8),
    ADD(u8),
    SUB(u8),
    INP,
    OUT,
    HLT,
    BRA(u8),
    BRZ(u8),
    BRP(u8),
    DAT(i16),
}

impl Command {
    pub fn get_byte_data(&self) -> Result<i16, ParseIntError> {
        let memory_value = match self {
            Command::LDA(value) => format!("5{:02}", value).parse::<i16>(),
            Command::STA(value) => format!("3{:02}", value).parse::<i16>(),
            Command::ADD(value) => format!("1{:02}", value).parse::<i16>(),
            Command::SUB(value) => format!("2{:02}", value).parse::<i16>(),
            Command::INP => Ok(901i16),
            Command::OUT => Ok(902i16),
            Command::HLT => Ok(000i16),
            Command::BRA(value) => format!("6{:02}", value).parse::<i16>(),
            Command::BRZ(value) => format!("7{:02}", value).parse::<i16>(),
            Command::BRP(value) => format!("8{:02}", value).parse::<i16>(),
            Command::DAT(value) => format!("{}", value).parse::<i16>(),
        };

        memory_value
    }

    pub fn from_token_location_literal(token:Token, location:u8) -> Command {
        match token {
            Token::Load => Command::LDA(location),
            Token::Store => Command::STA(location),
            Token::Add => Command::ADD(location),
            Token::Subtract => Command::SUB(location),
            Token::BranchAll => Command::BRA(location),
            Token::BranchZero => Command::BRZ(location),
            Token::BranchZeroPos => Command::BRP(location),
            _ => unreachable!()
        }
    }
}
