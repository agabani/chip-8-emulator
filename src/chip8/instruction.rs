#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    /// 00E0
    ClearScreen,
    /// 00EE
    Return,
    /// 1NNN
    Jump { nnn: u16 },
    /// 2NNN
    Call { nnn: u16 },
    /// 6XNN
    SetRegister { x: u8, nn: u8 },
    /// 7XNN
    AddValueToRegister { x: u8, nn: u8 },
    /// 8XY0
    Set { x: u8, y: u8 },
    /// 9XY0
    SkipIfNotEqual { x: u8, y: u8 },
    /// ANNN
    SetIndexRegister { nnn: u16 },
    /// DXYN
    DisplayDraw { x: u8, y: u8, n: u8 },
    /// FX65
    LoadMemory { x: u8 },
}

impl Instruction {
    pub(super) fn parse(bytes: [u8; 2]) -> Instruction {
        let nibbles = [
            (bytes[0] & 0xF0) >> 4,
            bytes[0] & 0x0F,
            (bytes[1] & 0xF0) >> 4,
            bytes[1] & 0x0F,
        ];

        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Instruction::ClearScreen,
            [0x0, 0x0, 0xE, 0xE] => Instruction::Return,
            [0x1, n2, n3, n4] => Instruction::Jump {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0x2, n2, n3, n4] => Instruction::Call {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0x6, n2, n3, n4] => Instruction::SetRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x7, n2, n3, n4] => Instruction::AddValueToRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x8, n2, n3, 0x0] => Instruction::Set { x: n2, y: n3 },
            [0x9, n2, n3, 0x0] => Instruction::SkipIfNotEqual { x: n2, y: n3 },
            [0xA, n2, n3, n4] => Instruction::SetIndexRegister {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0xD, n2, n3, n4] => Instruction::DisplayDraw {
                x: n2,
                y: n3,
                n: n4,
            },
            [0xF, n2, 0x6, 0x5] => Instruction::LoadMemory { x: n2 },
            [n1, n2, n3, n4] => todo!("{:1X} {:1X} {:1X} {:1X}", n1, n2, n3, n4),
        }
    }
}
