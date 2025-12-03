pub mod i8080;
pub mod pointer;
pub mod register;


use std::fmt;

use crate::i8080::{I8080};


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
    use crate::pointer::Pointer;

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
        assert_eq!(cpu.cycles == 5, true);
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
        assert_eq!(cpu.cycles == 5, true);
    }

    #[test]
    fn test_mov3() {
        let mut cpu = I8080::new();
        cpu.flags.0 = 0b00010000;
        cpu.a.0 = 0x33;
        cpu.d.0 = 0xff;
        // mov C, B
        cpu.mov(0x57);
        assert_eq!(cpu.d.0 == cpu.a.0, true);
        assert_eq!(cpu.cycles == 5, true);
    }

    #[test]
    fn test_mov4() {
        let mut cpu = I8080::new();
        cpu.pc.0 = 0xff;
        cpu.a.0 = 0x33;
        cpu.mov(0x77);
        println!("{:?}", cpu);
        assert_eq!(cpu.mem[0xff] == cpu.a.0, true);
        assert_eq!(cpu.cycles == 7, true);
    }

    #[test]
    fn test_mov5() {
        let mut cpu = I8080::new();
        cpu.h.0 = 0x02;
        cpu.l.0 = 0x34;
        let pointer : Pointer = Pointer(0x0234);
        // mem[0x234] = 0xff
        pointer.store(&mut cpu.mem, 0xff);
        // MOV B, M
        cpu.mov(0x46);
        assert_eq!(cpu.b.0 == 0xff, true);
        assert_eq!(cpu.cycles == 7, true);
    }

    #[test]
    fn test_mov6() {
        let mut cpu = I8080::new();
        cpu.l.0 = 0xcd;
        cpu.pc.0 = 0xabcd;
        let pointer : Pointer = Pointer(0xabcd);
        // mov M, L
        cpu.mov(0x75);
        let val: u8 = pointer.load(&cpu.mem);
        assert_eq!(val == 0xcd, true);
        assert_eq!(cpu.cycles == 7, true);
    }

    #[test]
    fn test_lxi1() {
        let mut cpu = I8080::new();
        let mut pointer = Pointer(0xcafe);
        pointer.store(&mut cpu.mem, 0xab);
        pointer.inc();
        pointer.store(&mut cpu.mem, 0xcd);
        cpu.pc.0 = 0xcafe;
        // lxi B
        cpu.lxi(0x01);
        cpu.show_registers();
        println!("low 0x{:02x} and high 0x{:02x}", cpu.mem[0xcafe], cpu.mem[0xcaff]);
        assert_eq!(cpu.b.0 == 0xcd, true);
        assert_eq!(cpu.c.0 == 0xab, true);
        assert_eq!(cpu.cycles == 10, true);
    }

    #[test]
    fn test_lxi2() {
        let mut cpu = I8080::new();
        let mut pointer = Pointer(0xcafe);
        pointer.store(&mut cpu.mem, 0xab);
        pointer.inc();
        pointer.store(&mut cpu.mem, 0xcd);
        cpu.pc.0 = 0xcafe;
        // lxi D
        cpu.lxi(0x11);
        cpu.show_registers();
        println!("low 0x{:02x} and high 0x{:02x}", cpu.mem[0xcafe], cpu.mem[0xcaff]);
        assert_eq!(cpu.d.0 == 0xcd, true);
        assert_eq!(cpu.e.0 == 0xab, true);
        assert_eq!(cpu.cycles == 10, true);
    }

    #[test]
    fn test_lxi3() {
        let mut cpu = I8080::new();
        let mut pointer = Pointer(0xcafe);
        pointer.store(&mut cpu.mem, 0xab);
        pointer.inc();
        pointer.store(&mut cpu.mem, 0xcd);
        cpu.pc.0 = 0xcafe;
        // lxi H
        cpu.lxi(0x21);
        cpu.show_registers();
        println!("low 0x{:02x} and high 0x{:02x}", cpu.mem[0xcafe], cpu.mem[0xcaff]);
        assert_eq!(cpu.h.0 == 0xcd, true);
        assert_eq!(cpu.l.0 == 0xab, true);
        assert_eq!(cpu.cycles == 10, true);
    }


    #[test]
    fn test_lxi4() {
        let mut cpu = I8080::new();
        let mut pointer = Pointer(0xcafe);
        pointer.store(&mut cpu.mem, 0xab);
        pointer.inc();
        pointer.store(&mut cpu.mem, 0xcd);
        cpu.pc.0 = 0xcafe;
        // lxi SP
        cpu.lxi(0x31);
        cpu.show_registers();
        println!("low 0x{:02x} and high 0x{:02x}", cpu.mem[0xcafe], cpu.mem[0xcaff]);
        assert_eq!(cpu.sp.0 == 0xcdab, true);
        assert_eq!(cpu.cycles == 10, true);
    }
}
