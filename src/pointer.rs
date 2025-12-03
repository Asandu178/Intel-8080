#[derive(Debug, Default)]
pub struct Pointer(pub u16);

impl Pointer {
    pub fn load(&self, mem: &[u8]) -> u8 {
        mem[self.0 as usize]
    }
    pub fn store(&self, mem: &mut [u8], byte: u8) {
        mem[self.0 as usize] = byte;
    }
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}