use core::panic;
use std::{fmt};

use crate::{pointer::Pointer, register::Register};

pub struct I8080 {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub flags: Register,
    pub h: Register,
    pub l: Register,
    pub pc: Pointer,
    pub sp: Pointer,
    pub cycles : u64,
    pub mem : [u8; 64 * 1024],
}

impl fmt::Debug for I8080 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I8080")
        .field("a", &self.a)
        .field("b", &self.b)
        .field("c", &self.c)
        .field("d", &self.d)
        .field("e", &self.e)
        .field("flags", &self.flags)
        .field("h", &self.h)
        .field("l", &self.l)
        .field("pc", &self.pc)
        .field("sp", &self.sp)
        .field("cycles", &self.cycles)
        .finish()
    }
}

impl I8080 {
    pub fn new() -> Self {
        Self {
            a: Register::default(),
            b: Register::default(),
            c: Register::default(),
            d: Register::default(),
            e: Register::default(),
            flags: Register::default(),
            h: Register::default(),
            l: Register::default(),
            pc: Pointer::default(),
            sp: Pointer::default(),
            mem : [0 ; 64 * 1024],
            cycles : 0,
        }
    }

    pub fn show_registers(&self) {
        println!(
            "A: 0x{:02x} B: 0x{:02x} C: 0x{:02x} D: 0x{:02x} E: 0x{:02x}",
            self.a.0, self.b.0, self.c.0, self.d.0, self.e.0
        );
        println!(
            "H: 0x{:02x} L: 0x{:02x} FLAGS: 0x{:08b}",
            self.h.0, self.l.0, self.flags.0
        );
        println!(
            "PC: 0x{:04x} SP: 0x{:04x}",
            self.pc.0, self.sp.0
        );
    }

    pub fn mov(&mut self, opcode: u8) {
        // stole this beautiful macro from XAMPPRocky's similar project
        // still learning macros so please dont blame me
        macro_rules! mov {
            ($x: ident, $b: expr, $c: expr, $d: expr, $e: expr, $h: expr, $l: expr, $hl: expr, $a: expr) => {{
                self.$x.0 = match opcode {
                    $b => self.b.0,
                    $c => self.c.0,
                    $d => self.d.0,
                    $e => self.e.0,
                    $h => self.h.0,
                    $l => self.l.0,
                    $hl => self.h.register_pair(&self.l).load(&self.mem),
                    $a => self.a.0,
                    _ => panic!("Eroare!"),
                };
                // normal MOV operations take 5 cycles
                self.cycles += 5;
                
                // for moves involving M
                if opcode == $hl {
                    self.cycles +=2;
                }
            }};
        }

        macro_rules! movm {
            ($b: expr, $c: expr, $d: expr, $e: expr, $h: expr, $l: expr, $a: expr) => {{
                match opcode {
                    $b => self.pc.store(&mut self.mem, self.b.0),
                    $c => self.pc.store(&mut self.mem, self.c.0),
                    $d => self.pc.store(&mut self.mem, self.d.0),
                    $e => self.pc.store(&mut self.mem, self.e.0),
                    $h => self.pc.store(&mut self.mem, self.h.0),
                    $l => self.pc.store(&mut self.mem, self.l.0),
                    $a => self.pc.store(&mut self.mem, self.a.0),
                    _ => panic!("Eroare"),
                }
                // for moves involving M
                self.cycles += 7;
            }};
        }

        match opcode {
            0x78..=0x7f => mov!(a, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f),
            0x40..=0x47 => mov!(b, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47),
            0x48..=0x4f => mov!(c, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f),
            0x50..=0x57 => mov!(d, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57),
            0x58..=0x5f => mov!(e, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f),
            0x60..=0x67 => mov!(h, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67),
            0x68..=0x6f => mov!(l, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f),
            0x70..=0x75 | 0x77 => movm!(0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x77),
            _ => panic!("Eroare!"),
        }
    }

    pub fn lxi(&mut self, opcode: u8) {
        self.cycles += 10;

        macro_rules! lxi {
            ($h: ident, $l: ident) => {
                {
                    self.$l.0 = self.pc.load(&self.mem);
                    self.pc.inc();
                    self.$h.0 = self.pc.load(&self.mem);
                    self.pc.inc();
                }
            };
        }

        match opcode {
            0x01 => lxi!(b, c),
            0x11 => lxi!(d, e),
            0x21 => lxi!(h, l),
            0x31 => {
                let l = self.pc.load(&self.mem);
                self.pc.inc();
                let h = self.pc.load(&self.mem);
                self.pc.inc();
                self.sp.0 = (h as u16) << 8 | (l as u16);
            }
            _ => panic!("Eroare!")
        }
    }
}