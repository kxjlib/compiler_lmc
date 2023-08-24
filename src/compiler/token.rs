use super::command::Command;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(in crate::compiler) enum Token {
    Load,          // LDA 5xx
    Store,         // STA 3xx
    Add,           // ADD 1xx
    Subtract,      // SUB 2xx
    Input,         // INP 901
    Output,        // OUT 902
    End,           // HLT 000
    BranchAll,     // BRA 6xx
    BranchZero,    // BRZ 7xx
    BranchZeroPos, // BRP 8xx
    DataStore,     // DAT N/A
    Endline,
    IntLiteral(i16),
    IntLiteralNoData,
}

impl Token {
    pub fn next_valid(&self, is_first_token: bool) -> Vec<Self> {
        match self {
            Self::Load
            | Self::Store
            | Self::Add
            | Self::Subtract
            | Self::BranchAll
            | Self::BranchZero
            | Self::BranchZeroPos => vec![Self::IntLiteralNoData],
            Self::IntLiteral(_) | Self::End | Self::Output | Self::Input => {
                vec![Self::Endline]
            }
            Self::DataStore => {
                if is_first_token {
                    vec![Self::IntLiteralNoData]
                } else {
                    vec![Self::IntLiteralNoData, Self::Endline]
                }
            }

            Self::Endline => vec![
                Self::Load,
                Self::Store,
                Self::Add,
                Self::Subtract,
                Self::Input,
                Self::Output,
                Self::End,
                Self::BranchAll,
                Self::BranchZero,
                Self::BranchZeroPos,
                Self::DataStore,
                Self::Endline,
            ],
            Self::IntLiteralNoData => unreachable!(),
        }
    }
}
