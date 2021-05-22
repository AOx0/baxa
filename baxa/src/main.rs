use baxa_core::{error, baxa, bxvm};
use baxa_core::bxvm::bytecode::*;

fn main() -> Result<(), error::Error>{
    // match baxa::file_manager::load_from("lol.bx") {
    //     Ok(contents) => println!("{}", contents),
    //     Err(error) => println!("{}", error)
    // }

    let bytecode: [Bytecode; 5] = [
            Bytecode::Instruction(PRINT), Bytecode::Data(Value::BxStr((&"Hola Mundo").to_string())),
            Bytecode::Instruction(ADD), Bytecode::Data(Value::BxInt(9)), Bytecode::Data(Value::BxInt(9))
    ];

    baxa::file_manager::write_contents_to("lol.bx", &compile(&bytecode))?;


    //let mut vm = bxvm::vm::Vm::new();
    //vm.interpret_bytecode(&bytecode)?;
    Ok(())
}
