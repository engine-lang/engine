mod constants;
mod compiler;

use crate::compiler::compile;


fn main() {
    let result = compile();
    if result.is_err(){
        panic!("{}", result.unwrap_err());
    }
}
