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
    pub fn set_bc(&mut self, value: u16) {
        let values = Self::split_bytes(value);
        self.b = values.0;
        self.c = values.1;
    }

    #[allow(dead_code)]
    pub fn set_de(&mut self, value: u16) {
        let values = Self::split_bytes(value);
        self.d = values.0;
        self.e = values.1;
    }

    #[allow(dead_code)]
    pub fn set_hl(&mut self, value: u16) {
        let values = Self::split_bytes(value);
        self.h = values.0;
        self.l = values.1;
    }

    pub fn bc(&self) -> u16 {
        Self::combine_bytes(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        Self::combine_bytes(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        Self::combine_bytes(self.h, self.l)
    }

    pub fn z(&self) -> u8 {
        self.f >> 7
    }

    pub fn set_flags(&mut self, z: u8, n: u8, h: u8, c: u8) {
        self.f = 0 | (z << 7) | (n << 6) | (h << 5) | (c << 4);
    }

    fn split_bytes(value: u16) -> (u8, u8) {
        (((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
    }

    fn combine_bytes(upper: u8, lower: u8) -> u16 {
        ((upper as u16) << 8) | lower as u16
    }
}
