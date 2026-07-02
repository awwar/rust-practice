mod operation;
mod program;
mod value;
mod vm;

pub use crate::vm::operation::{get_op_executable, Operation};
pub use crate::vm::program::Program;
pub use crate::vm::value::Value;
pub use crate::vm::vm::{Stack, VM};
