use super::{display::Display, register::Register};

#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    /// 00E0
    CLS(CLS),
    /// 00EE
    RET(RET),
    /// 0NNN
    SYS(SYS),
    /// 1NNN
    Jump { nnn: u16 },
    /// 2NNN
    Call { nnn: u16 },
    /// 3XNN
    SkipIfEqual1 { x: u8, nn: u8 },
    /// 4XNN
    SkipIfNotEqual1 { x: u8, nn: u8 },
    /// 5XY0
    SkipIfEqual2 { x: u8, y: u8 },
    /// 6XNN
    SetRegister { x: u8, nn: u8 },
    /// 7XNN
    AddValueToRegister { x: u8, nn: u8 },
    /// 8XY0
    Set { x: u8, y: u8 },
    /// 8XY1
    BinaryOr { x: u8, y: u8 },
    /// 8XY2
    BinaryAnd { x: u8, y: u8 },
    /// 8XY3
    LogicalXor { x: u8, y: u8 },
    /// 8XY4
    Add { x: u8, y: u8 },
    /// 8XY5
    SubtractRightFromLeft { x: u8, y: u8 },
    /// 8XY6
    ShiftRight { x: u8, y: u8 },
    /// 8XYE
    ShiftLeft { x: u8, y: u8 },
    /// 9XY0
    SkipIfNotEqual2 { x: u8, y: u8 },
    /// ANNN
    SetIndexRegister { nnn: u16 },
    /// BNNN
    JumpWithOffset { nnn: u16 },
    /// CXNN
    Random { x: u8, nn: u8 },
    /// DXYN
    DisplayDraw { x: u8, y: u8, n: u8 },
    /// EX9E
    SkipIfKeyPressed { x: u8 },
    /// EXA1
    SkipIfKeyNotPressed { x: u8 },
    /// FX07
    SetCurrentDelayTimerValueToRegister { x: u8 },
    /// FX0A
    GetKey { x: u8 },
    /// FX15
    SetDelayTimer { x: u8 },
    /// FX18
    SetSoundTimer { x: u8 },
    /// FX1E
    AddToIndex { x: u8 },
    /// FX29
    LoadFont { x: u8 },
    /// FX33
    BinaryCodedDecimalConversion { x: u8 },
    /// FX55
    StoreMemory { x: u8 },
    /// FX65
    LoadMemory { x: u8 },
}

/// 00E0 - CLS
///
/// Clear the display.
#[derive(Debug, PartialEq)]
pub(super) struct CLS;

/// 00EE - RET
///
/// Return from a subroutine.
///
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
#[derive(Debug, PartialEq)]
pub(super) struct RET;

/// 0nnn - SYS addr
///
/// Jump to a machine code routine at nnn.
///
/// This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
#[derive(Debug, PartialEq)]
pub(super) struct SYS {
    nnn: u16,
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
            [0x0, 0x0, 0xE, 0x0] => Instruction::CLS(CLS::new()),
            [0x0, 0x0, 0xE, 0xE] => Instruction::RET(RET::new()),
            [0x0, n2, n3, n4] => Instruction::SYS(SYS::new(
                (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            )),
            [0x1, n2, n3, n4] => Instruction::Jump {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0x2, n2, n3, n4] => Instruction::Call {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0x3, n2, n3, n4] => Instruction::SkipIfEqual1 {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x4, n2, n3, n4] => Instruction::SkipIfNotEqual1 {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x5, n2, n3, 0x0] => Instruction::SkipIfEqual2 { x: n2, y: n3 },
            [0x6, n2, n3, n4] => Instruction::SetRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x7, n2, n3, n4] => Instruction::AddValueToRegister {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0x8, n2, n3, 0x0] => Instruction::Set { x: n2, y: n3 },
            [0x8, n2, n3, 0x1] => Instruction::BinaryOr { x: n2, y: n3 },
            [0x8, n2, n3, 0x2] => Instruction::BinaryAnd { x: n2, y: n3 },
            [0x8, n2, n3, 0x3] => Instruction::LogicalXor { x: n2, y: n3 },
            [0x8, n2, n3, 0x4] => Instruction::Add { x: n2, y: n3 },
            [0x8, n2, n3, 0x5] => Instruction::SubtractRightFromLeft { x: n2, y: n3 },
            [0x8, n2, n3, 0x6] => Instruction::ShiftRight { x: n2, y: n3 },
            [0x8, n2, n3, 0xE] => Instruction::ShiftLeft { x: n2, y: n3 },
            [0x9, n2, n3, 0x0] => Instruction::SkipIfNotEqual2 { x: n2, y: n3 },
            [0xA, n2, n3, n4] => Instruction::SetIndexRegister {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0xB, n2, n3, n4] => Instruction::JumpWithOffset {
                nnn: (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4)),
            },
            [0xC, n2, n3, n4] => Instruction::Random {
                x: n2,
                nn: (n3 << 4) + n4,
            },
            [0xD, n2, n3, n4] => Instruction::DisplayDraw {
                x: n2,
                y: n3,
                n: n4,
            },
            [0xE, n2, 0x9, 0xE] => Instruction::SkipIfKeyPressed { x: n2 },
            [0xE, n2, 0xA, 0x1] => Instruction::SkipIfKeyNotPressed { x: n2 },
            [0xF, n2, 0x0, 0x7] => Instruction::SetCurrentDelayTimerValueToRegister { x: n2 },
            [0xF, n2, 0x0, 0xA] => Instruction::GetKey { x: n2 },
            [0xF, n2, 0x1, 0x5] => Instruction::SetDelayTimer { x: n2 },
            [0xF, n2, 0x1, 0x8] => Instruction::SetSoundTimer { x: n2 },
            [0xF, n2, 0x1, 0xE] => Instruction::AddToIndex { x: n2 },
            [0xF, n2, 0x2, 0x9] => Instruction::LoadFont { x: n2 },
            [0xF, n2, 0x3, 0x3] => Instruction::BinaryCodedDecimalConversion { x: n2 },
            [0xF, n2, 0x5, 0x5] => Instruction::StoreMemory { x: n2 },
            [0xF, n2, 0x6, 0x5] => Instruction::LoadMemory { x: n2 },
            [n1, n2, n3, n4] => todo!("{:1X} {:1X} {:1X} {:1X}", n1, n2, n3, n4),
        }
    }
}

impl CLS {
    pub(super) fn new() -> CLS {
        CLS
    }

    pub(super) fn execute(&self, register: &mut Register, display: &mut Display) {
        display.clear_screen();
        register.increment_program_counter();
    }
}

impl RET {
    pub(super) fn new() -> RET {
        RET
    }

    pub(super) fn execute(&self, register: &mut Register) {
        let program_counter = register.pop_stack();
        register.set_program_counter(program_counter);
        register.increment_program_counter();
    }
}

impl SYS {
    pub(super) fn new(nnn: u16) -> SYS {
        SYS { nnn }
    }

    pub(super) fn execute(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cls() {
        // Arrange
        let mut display = Display::new();
        let mut register = Register::new();
        let instruction = CLS::new();

        // Act
        instruction.execute(&mut register, &mut display);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(display.is_pixel_on(0, 0), false);
        assert_eq!(display.is_pixel_on(63, 0), false);
        assert_eq!(display.is_pixel_on(0, 31), false);
        assert_eq!(display.is_pixel_on(63, 31), false);
    }

    #[test]
    fn test_ret() {
        // Arrange
        let mut register = Register::new();
        register.push_stack(0x400);
        register.push_stack(0x600);

        let instruction = RET::new();

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x602);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x402);
    }

    #[test]
    #[should_panic]
    fn test_sys() {
        // Arrange
        let instruction = SYS::new(000);

        // Act
        instruction.execute();
    }
}
