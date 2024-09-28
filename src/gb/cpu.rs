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
        let values = split_bytes(value);
        self.b = values.0;
        self.c = values.1;
    }

    pub fn set_de(&mut self, value: u16) {
        let values = split_bytes(value);
        self.d = values.0;
        self.e = values.1;
    }

    pub fn set_hl(&mut self, value: u16) {
        let values = split_bytes(value);
        self.h = values.0;
        self.l = values.1;
    }

    pub fn bc(&self) -> u16 {
        combine_bytes(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        combine_bytes(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        combine_bytes(self.h, self.l)
    }

    pub fn z(&self) -> u8 {
        self.f >> 7
    }

    pub fn set_flags(&mut self, z: u8, n: u8, h: u8, c: u8) {
        self.f = 0 | (z << 7) | (n << 6) | (h << 5) | (c << 4);
    }

    pub fn registers<'a>(&'a self) -> Registers<'a> {
        Registers { cpu: self, idx: 0 }
    }
}

fn split_bytes(value: u16) -> (u8, u8) {
    (((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
}

fn combine_bytes(upper: u8, lower: u8) -> u16 {
    ((upper as u16) << 8) | lower as u16
}

pub struct Registers<'a> {
    cpu: &'a LR35902,
    idx: usize,
}

impl<'a> Iterator for Registers<'a> {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        let register = match self.idx {
            0 => Register { name: "a", value: self.cpu.a },
            1 => Register { name: "b", value: self.cpu.b },
            2 => Register { name: "c", value: self.cpu.c },
            3 => Register { name: "d", value: self.cpu.d },
            4 => Register { name: "e", value: self.cpu.e },
            5 => Register { name: "f", value: self.cpu.f },
            6 => Register { name: "h", value: self.cpu.h },
            7 => Register { name: "l", value: self.cpu.l },
            _ => return None,
        };
        self.idx += 1;
        Some(register)
    }
}

pub struct Register {
    pub name: &'static str,
    pub value: u8,
}
