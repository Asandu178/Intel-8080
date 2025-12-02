use std::{fmt, iter::Cycle};

use crate::{pointer::Pointer, register::Register};

pub enum FromAddr {
    None,
    B,
    D,
    H,
}

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

    fn read_byte_from_memory(&mut self, origin : FromAddr) -> u8 {
        // for now i'll say that every read from memory does 2 cycles, since
        // i've only implemented mov at it seems to be the case might be forced to find a better
        // approach later
        self.cycles += 2;
        
        let addr : u16 = match origin {
            FromAddr::B => (self.b.0 as u16) << 8 | (self.c.0 as u16),
            FromAddr::D => (self.d.0 as u16) << 8 | (self.e.0 as u16),
            FromAddr::H => (self.h.0 as u16) << 8 | (self.l.0 as u16),
            FromAddr::None => self.pc.0,
        };

        self.mem[addr as usize]
    }

    pub fn mov(&mut self, opcode: u8) {
        // stole this beautiful macro from XAMPPRocky's similar project
        // still learning macros so please dont blame me
        macro_rules! mov {
            ($x: ident, $b: expr, $c: expr, $d: expr, $e: expr, $h: expr, $l: expr, $hl: expr, $a: expr) => {
                self.$x.0 = match opcode {
                    $b => self.b.0,
                    $c => self.c.0,
                    $d => self.d.0,
                    $e => self.e.0,
                    $h => self.h.0,
                    $l => self.l.0,
                    $hl => self.read_byte_from_memory(FromAddr::H),
                    $a => self.a.0,
                    _ => panic!("Eroare!"),
                }
            };
        }

        // every mov does adds at least 5 cycles, those involving memory add 7
        self.cycles += 5;

        match opcode {
            0x78..0x7f => mov!(a, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f),
            0x40..0x47 => mov!(b, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47),
            0x48..0x4f => mov!(c, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f),
            0x50..0x57 => mov!(d, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57),
            0x58..0x5f => mov!(e, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f),
            0x60..0x67 => mov!(h, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67),
            0x68..0x6f => mov!(l, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f),
            // TREAT MOV(M, REG) 0x70..0x75 | 0x77 => ...



            _ => panic!("Not yet implemented"),
        }
    }
}