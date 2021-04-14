pub struct LR35902 {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

impl LR35902 {
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn z(&self) -> u8 {
        self.f >> 7
    }

    pub fn set_flags(&mut self, z: u8, n: u8, h: u8, c: u8) {
        self.f = 0 | (z << 7) | (n << 6) | (h << 5) | (c << 4);
    }
}