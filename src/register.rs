use crate::pointer::Pointer;

#[derive(Debug, Default)]
pub struct Register(pub u8);

impl Register {
    pub fn register_pair(&self, low: &Register) -> Pointer {
        let addr: u16 = (self.0 as u16) << 8 | (low.0 as u16);
        Pointer(addr)
    }
}