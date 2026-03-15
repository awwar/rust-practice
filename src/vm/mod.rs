mod vm;
mod operation;
mod program;
mod value;


pub use crate::vm::vm::{VM, Stack};
pub use crate::vm::program::{Program};
pub use crate::vm::value::{Value};
pub use crate::vm::operation::{get_op_executable, Operation};
