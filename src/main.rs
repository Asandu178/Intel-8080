pub mod i8080;
mod pointer;
mod register;


use std::fmt;

use crate::i8080::I8080;


fn main() {
    let mut cpu = I8080::new();
    cpu.flags.0 = 0b00010000;
    cpu.b.0 = 0x33;
    cpu.c.0 = 0xff;
    cpu.mov(0x79);
    cpu.show_registers();
}

// too lazy to do proper testing rn

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_mov1() {
        let mut cpu = I8080::new();
        cpu.flags.0 = 0b00010000;
        cpu.b.0 = 0x33;
        cpu.c.0 = 0xff;
        // mov A, B
        cpu.mov(0x78);
        assert_eq!(cpu.a.0 == cpu.b.0, true);
    }
    
    #[test]
    fn test_mov2() {
        let mut cpu = I8080::new();
        cpu.flags.0 = 0b00010000;
        cpu.b.0 = 0x33;
        cpu.c.0 = 0xff;
        // mov C, B
        cpu.mov(0x48);
        assert_eq!(cpu.c.0 == cpu.b.0, true);
    }
}
