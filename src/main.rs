use std::io;
use std::io::prelude::*;
use std::fs::File;

// #[macro_use] extern crate log;

fn disassemble(opcode: u16) {
    let op1 = ((opcode & 0xF000) >> 12) as u8;
    let op2 = ((opcode & 0x0F00) >> 8) as u8;
    let op3 = ((opcode & 0x00F0) >> 4) as u8;
    let op4 = (opcode & 0x000F) as u8;

    let NNN = opcode & 0x0FFF;
    let NN = (opcode & 0x00FF) as u8;

    print!("{:#06X?}: ", opcode);

    match (op1, op2, op3, op4) {
        (0x0, 0x0, 0xE, 0x0) => println!("Clear screen"),
        (0x0, 0x0, 0xE, 0xE) => println!("Return from a subroutine"),
        (0x0, _, _, _) => println!("Execute machine language subroutine at address {}", NNN),
        (0x1, _, _, _) => println!("Jump to address {}", NNN),
        (0x2, _, _, _) => println!("Execute subroutine at address {}", NNN),
        (0x3, X, _, _) => println!("Skip the following instruction if V{} == {}", X, NN),
        (0x4, X, _, _) => println!("Skip the following instruction if V{} != {}", X, NN),
        (0x5, X, Y, 0) => println!("Skip the following instruction if V{} == V{}", X, Y),
        (0x6, X, _, _) => println!("Store {} in register V{}", NN, X),
        (0x7, X, _, _) => println!("Add {} to register V{}", NN, X),
        (0x8, X, Y, 0x0) => println!("Store V{} in V{}", Y, X),
        (0x8, X, Y, 0x1) => println!("Set V{} to V{} | V{}", X, X, Y),
        (0x8, X, Y, 0x2) => println!("Set V{} to V{} & V{}", X, X, Y),
        (0x8, X, Y, 0x3) => println!("Set V{} to V{} ^ V{}", X, X, Y),
        (0x8, X, Y, 0x4) => println!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", Y, X),
        (0x8, X, Y, 0x5) => println!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", Y, X),
        (0x8, X, Y, 0x6) => println!("Store the value of register V{} shifted right one bit in register V{}\n\tSet register VF to the least significant bit prior to the shift", Y, X),
        (0x8, _, _, opt) => println!("UNRECOGNIZED OPTION: {}", opt),
        (_, _, _, _) => println!("UNIMPLEMENTED OP")
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("programs/PONG")?;

    let mut buffer = [0; 2];

    let mut bytes_read = f.read(&mut buffer);

    while bytes_read.is_ok() {
        match bytes_read {
            Ok(n) => if n == 0 {
                break;
            } else {
                let opcode = ((buffer[0] as u16) << 8) | buffer[1] as u16;
                disassemble(opcode);
                // println!("{:#06X?}", number);
            }
            _ => continue
        }

        bytes_read = f.read(&mut buffer);
        // println!("{:#X?}", buffer);
    }


    Ok(())
}
