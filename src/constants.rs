pub const VERSION: &'static str = "v0.1.0";
pub const VARIABLE_MAX_LENGTH: i8 = 100;
pub const INT_NUMBER_MAX_LENGTH: i8 = 18;
pub const DOUBLE_NUMBER_MAX_LENGTH: i8 = 18;


pub mod compiler{
    pub const BYTECODE_SPACE_STRING_LENGTH: i8 = 80;
}


#[derive(Clone)]
pub enum Mode {
    Compiler,
    ByteCodeGenerator,
    Interpreter,
    VirtualMachine,
}

impl std::fmt::Debug for Mode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler => write!(f, "Engine Compiler"),
            Self::ByteCodeGenerator => write!(f, "Engine ByteCodeGenerator"),
            Self::Interpreter => write!(f, "Engine Interpreter"),
            Self::VirtualMachine => write!(f, "Engine VM"),
        }
    }
}

impl std::fmt::Display for Mode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler => write!(f, "Engine Compiler"),
            Self::ByteCodeGenerator => write!(f, "Engine ByteCodeGenerator"),
            Self::Interpreter => write!(f, "Engine Interpreter"),
            Self::VirtualMachine => write!(f, "Engine VM"),
        }
    }
}
