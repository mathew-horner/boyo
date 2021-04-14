const OPCODE_MAP: [[Option<Opcode>; 16]; 16] = [
    [Some(Opcode::NOP),     None, None, None,                   None, None, None, Some(Opcode::RLCA), None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [Some(Opcode::LD_HL_B), None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, Some(Opcode::JP_a16),   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
    [None,                  None, None, None,                   None, None, None, None,               None, None, None, None, None, None, None, None],
];

const OPCODE_SIZES: [[u8; 16]; 16] = [
    [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Opcode {
    NOP,
    RLCA,
    LD_HL_B,
    JP_a16,
}

impl Opcode {
    pub fn parse(opcode: u8) -> Option<Self> {
        let (row, col) = Self::get_indices(opcode);
        OPCODE_MAP[row][col].clone()
    }

    pub fn size(opcode: u8) -> u8 {
        let (row, col) = Self::get_indices(opcode);
        OPCODE_SIZES[row][col]
    }

    fn get_indices(opcode: u8) -> (usize, usize) {
        ((opcode >> 4) as usize, (opcode & 0xF) as usize)
    }
}