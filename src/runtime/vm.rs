use super::error;
use crate::common::{module::*, program::Program};

pub struct VM {
    program: Program,
}

impl VM {
    pub fn new(program: Program) -> VM {
        VM { program: program }
    }

    pub fn run(&self) -> Result<(), idioma::Error> {
        match self.program.get(&String::from("Main")) {
            None => Err(idioma::error(error::NO_MAIN_MODULE)),
            Some(main) => self.execute(main),
        }
    }

    pub fn boot(program: Program) -> Result<(), idioma::Error> {
        VM::new(program).run()
    }

    fn execute(&self, main: &Module) -> Result<(), idioma::Error> {
        match main.get(&String::from("main")) {
            None => Err(idioma::error(error::NO_MAIN_FUNCTION)),
            Some(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common_macros::hash_map;

    #[test]
    fn test_basic_program() -> Result<(), idioma::Error> {
        let module = hash_map! {
            "main".to_string() => Definition::Function {
            parameters: vec![],
            arguments: hash_map!{},
            body: Expression::Value(42),
        }};
        let program = hash_map! {
            "Main".to_string() => module,
        };
        VM::boot(program)
    }
}
