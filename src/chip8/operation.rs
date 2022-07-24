use super::{display::Display, register::Register};

#[derive(Debug, PartialEq)]
pub(super) enum Operation {
    /// 00E0
    CLS(CLS),
    /// 00EE
    RET(RET),
    /// 0NNN
    SYS(SYS),
    /// 1NNN
    JP(JP),
    /// 2NNN
    CALL(CALL),
    /// 3XNN
    SE1(SE1),
    /// 4XNN
    SNE1(SNE1),
    /// 5XY0
    SE2(SE2),
    /// 6XNN
    LD1(LD1),
    /// 7XNN
    ADD(ADD),
    /// 8XY0
    LD2(LD2),
    /// 8XY1
    OR(OR),
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

/// 1nnn - JP addr
///
/// Jump to location nnn.
///
/// The interpreter sets the program counter to nnn.
#[derive(Debug, PartialEq)]
pub(super) struct JP {
    nnn: u16,
}

/// 2nnn - CALL addr
///
/// Call subroutine at nnn.
///
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
#[derive(Debug, PartialEq)]
pub(super) struct CALL {
    nnn: u16,
}

/// 3xnn - SE Vx, byte
///
/// Skip next instruction if Vx = nn.
///
/// The interpreter compares register Vx to nn, and if they are equal, increments the program counter by 2.
#[derive(Debug, PartialEq)]
pub(super) struct SE1 {
    x: u8,
    nn: u8,
}

/// 4xnn - SNE Vx, byte
///
/// Skip next instruction if Vx != nn.
///
/// The interpreter compares register Vx to nn, and if they are not equal, increments the program counter by 2.
#[derive(Debug, PartialEq)]
pub(super) struct SNE1 {
    x: u8,
    nn: u8,
}

/// 5xy0 - SE Vx, Vy
///
/// Skip next instruction if Vx = Vy.
///
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
#[derive(Debug, PartialEq)]
pub(super) struct SE2 {
    x: u8,
    y: u8,
}

/// 6xnn - LD Vx, byte
///
/// Set Vx = nn.
///
/// The interpreter puts the value nn into register Vx.
#[derive(Debug, PartialEq)]
pub(super) struct LD1 {
    x: u8,
    nn: u8,
}

/// 7xnn - ADD Vx, byte
///
/// Set Vx = Vx + nn.
///
/// Adds the value nn to the value of register Vx, then stores the result in Vx.
#[derive(Debug, PartialEq)]
pub(super) struct ADD {
    x: u8,
    nn: u8,
}

/// 8xy0 - LD Vx, Vy
///
/// Set Vx = Vy.
///
/// Stores the value of register Vy in register Vx.
#[derive(Debug, PartialEq)]
pub(super) struct LD2 {
    x: u8,
    y: u8,
}

/// 8xy1 - OR Vx, Vy
///
/// Set Vx = Vx OR Vy.
///
/// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
#[derive(Debug, PartialEq)]
pub(super) struct OR {
    x: u8,
    y: u8,
}

impl Operation {
    pub(super) fn parse(bytes: [u8; 2]) -> Operation {
        let nibbles = nibble::from_bytes(bytes);

        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Operation::CLS(CLS::new()),
            [0x0, 0x0, 0xE, 0xE] => Operation::RET(RET::new()),
            [0x0, n2, n3, n4] => Operation::SYS(SYS::new(nibble::to_nnn(n2, n3, n4))),
            [0x1, n2, n3, n4] => Operation::JP(JP::new(nibble::to_nnn(n2, n3, n4))),
            [0x2, n2, n3, n4] => Operation::CALL(CALL::new(nibble::to_nnn(n2, n3, n4))),
            [0x3, n2, n3, n4] => Operation::SE1(SE1::new(nibble::to_n(n2), nibble::to_nn(n3, n4))),
            [0x4, n2, n3, n4] => {
                Operation::SNE1(SNE1::new(nibble::to_n(n2), nibble::to_nn(n3, n4)))
            }
            [0x5, n2, n3, 0x0] => Operation::SE2(SE2::new(n2, n3)),
            [0x6, n2, n3, n4] => Operation::LD1(LD1::new(nibble::to_n(n2), nibble::to_nn(n3, n4))),
            [0x7, n2, n3, n4] => Operation::ADD(ADD::new(nibble::to_n(n2), nibble::to_nn(n3, n4))),
            [0x8, n2, n3, 0x0] => Operation::LD2(LD2::new(nibble::to_n(n2), nibble::to_n(n3))),
            [0x8, n2, n3, 0x1] => Operation::OR(OR::new(nibble::to_n(n2), nibble::to_n(n3))),
            [0x8, n2, n3, 0x2] => Operation::BinaryAnd {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x8, n2, n3, 0x3] => Operation::LogicalXor {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x8, n2, n3, 0x4] => Operation::Add {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x8, n2, n3, 0x5] => Operation::SubtractRightFromLeft {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x8, n2, n3, 0x6] => Operation::ShiftRight {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x8, n2, n3, 0xE] => Operation::ShiftLeft {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0x9, n2, n3, 0x0] => Operation::SkipIfNotEqual2 {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
            },
            [0xA, n2, n3, n4] => Operation::SetIndexRegister {
                nnn: nibble::to_nnn(n2, n3, n4),
            },
            [0xB, n2, n3, n4] => Operation::JumpWithOffset {
                nnn: nibble::to_nnn(n2, n3, n4),
            },
            [0xC, n2, n3, n4] => Operation::Random {
                x: nibble::to_n(n2),
                nn: nibble::to_nn(n3, n4),
            },
            [0xD, n2, n3, n4] => Operation::DisplayDraw {
                x: nibble::to_n(n2),
                y: nibble::to_n(n3),
                n: nibble::to_n(n4),
            },
            [0xE, n2, 0x9, 0xE] => Operation::SkipIfKeyPressed {
                x: nibble::to_n(n2),
            },
            [0xE, n2, 0xA, 0x1] => Operation::SkipIfKeyNotPressed {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x0, 0x7] => Operation::SetCurrentDelayTimerValueToRegister {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x0, 0xA] => Operation::GetKey {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x1, 0x5] => Operation::SetDelayTimer {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x1, 0x8] => Operation::SetSoundTimer {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x1, 0xE] => Operation::AddToIndex {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x2, 0x9] => Operation::LoadFont {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x3, 0x3] => Operation::BinaryCodedDecimalConversion {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x5, 0x5] => Operation::StoreMemory {
                x: nibble::to_n(n2),
            },
            [0xF, n2, 0x6, 0x5] => Operation::LoadMemory {
                x: nibble::to_n(n2),
            },
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

impl JP {
    pub(super) fn new(nnn: u16) -> JP {
        JP { nnn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        register.set_program_counter(self.nnn);
    }
}

impl CALL {
    pub(super) fn new(nnn: u16) -> CALL {
        CALL { nnn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        register.push_stack(register.get_program_counter());
        register.set_program_counter(self.nnn);
    }
}

impl SE1 {
    pub(super) fn new(x: u8, nn: u8) -> SE1 {
        SE1 { x, nn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        if register.get_v_register(self.x) == self.nn {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }
}

impl SNE1 {
    pub(super) fn new(x: u8, nn: u8) -> SNE1 {
        SNE1 { x, nn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        if register.get_v_register(self.x) != self.nn {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }
}

impl SE2 {
    pub(super) fn new(x: u8, y: u8) -> SE2 {
        SE2 { x, y }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        if register.get_v_register(self.x) == register.get_v_register(self.y) {
            register.increment_program_counter();
        }
        register.increment_program_counter();
    }
}

impl LD1 {
    pub(super) fn new(x: u8, nn: u8) -> LD1 {
        LD1 { x, nn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        register.set_v_register(self.x, self.nn);
        register.increment_program_counter();
    }
}

impl ADD {
    pub(super) fn new(x: u8, nn: u8) -> ADD {
        ADD { x, nn }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        let (nn, _) = register.get_v_register(self.x).overflowing_add(self.nn);
        register.set_v_register(self.x, nn);
        register.increment_program_counter();
    }
}

impl LD2 {
    pub(super) fn new(x: u8, y: u8) -> LD2 {
        LD2 { x, y }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        register.set_v_register(self.x, register.get_v_register(self.y));
        register.increment_program_counter();
    }
}

impl OR {
    pub(super) fn new(x: u8, y: u8) -> OR {
        OR { x, y }
    }

    pub(super) fn execute(&self, register: &mut Register) {
        register.set_v_register(
            self.x,
            register.get_v_register(self.x) | register.get_v_register(self.y),
        );
        register.increment_program_counter();
    }
}

mod nibble {
    pub(super) fn from_bytes(bytes: [u8; 2]) -> [u8; 4] {
        [
            (bytes[0] & 0xF0) >> 4,
            bytes[0] & 0x0F,
            (bytes[1] & 0xF0) >> 4,
            bytes[1] & 0x0F,
        ]
    }

    pub(super) fn to_n(n: u8) -> u8 {
        n
    }

    pub(super) fn to_nn(n3: u8, n4: u8) -> u8 {
        (n3 << 4) + n4
    }

    pub(super) fn to_nnn(n2: u8, n3: u8, n4: u8) -> u16 {
        (u16::from(n2) << 8) + (u16::from(n3) << 4) + (u16::from(n4))
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

    #[test]
    fn test_jp() {
        // Arrange
        let mut register = Register::new();
        let instruction = JP::new(0x400);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x400);
    }

    #[test]
    fn test_call() {
        // Arrange
        let mut register = Register::new();
        let instruction = CALL::new(0x400);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x400);
        assert_eq!(register.pop_stack(), 0x200);
    }

    #[test]
    fn test_se1_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x2);
        let instruction = SE1::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x204);
    }

    #[test]
    fn test_se1_not_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x2);
        let instruction = SE1::new(0x4, 0x1);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
    }

    #[test]
    fn test_sne1_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x2);
        let instruction = SNE1::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
    }

    #[test]
    fn test_sne1_not_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x2);
        let instruction = SNE1::new(0x4, 0x1);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x204);
    }

    #[test]
    fn test_se2_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x7);
        register.set_v_register(0x2, 0x7);
        let instruction = SE2::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x204);
    }

    #[test]
    fn test_se2_not_equal() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0x7);
        register.set_v_register(0x2, 0x3);
        let instruction = SE2::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
    }

    #[test]
    fn test_ld1() {
        // Arrange
        let mut register = Register::new();
        let instruction = LD1::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(register.get_v_register(0x4), 0x2);
    }

    #[test]
    fn test_add() {
        // Arrange
        let mut register = Register::new();
        let instruction = ADD::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(register.get_v_register(0x4), 0x2);
        assert_eq!(register.get_v_register(0xF), 0x0);
    }

    #[test]
    fn test_add_overflow() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x4, 0xFF);
        let instruction = ADD::new(0x4, 0x2);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(register.get_v_register(0x4), 0x1);
        assert_eq!(register.get_v_register(0xF), 0x0);
    }

    #[test]
    fn test_ld2() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x7, 0x2);
        let instruction = LD2::new(0x4, 0x7);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(register.get_v_register(0x4), 0x2);
    }

    #[test]
    fn test_or() {
        // Arrange
        let mut register = Register::new();
        register.set_v_register(0x7, 0b01010000);
        register.set_v_register(0x4, 0b10100000);
        let instruction = OR::new(0x4, 0x7);

        // Act
        instruction.execute(&mut register);

        // Assert
        assert_eq!(register.get_program_counter(), 0x202);
        assert_eq!(register.get_v_register(0x4), 0b11110000);
    }
}
