#[derive(PartialEq, Clone, Copy)]
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
    NoToken,
}