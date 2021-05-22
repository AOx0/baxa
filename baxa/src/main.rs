use baxa_core::{error, baxa, bxvm};
use baxa_core::bxvm::bytecodes::*;

fn main() -> Result<(), error::Error>{
    // match baxa::file_manager::load_from("lol.bx") {
    //     Ok(contents) => println!("{}", contents),
    //     Err(error) => println!("{}", error)
    // }

    let bytecode = [
                PRINT, 22,
                STOP,
                SET, REG2, 9,
                ADD, REG1,       // reg1 = 9
                PRINT, REG1, 
                PRINT, REG2,
                GOIF, 3, REG1, REG2,
                PRINT, 33
    ];
    let mut vm = bxvm::vm::Vm::new();
    vm.interpret_bytecode(&bytecode)?;
    Ok(())
}
