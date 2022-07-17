#[derive(Debug, PartialEq)]
pub(super) enum Instruction {
    /// 00E0
    ClearScreen,
    /// 1NNN
    Jump { nnn: u16 },
    /// 6XNN
    SetRegister { x: u8, nn: u8 },
    /// 7XNN
    AddValueToRegister { x: u8, nn: u8 },
    /// ANNN
    SetIndexRegister { nnn: u16 },
    /// DXYN
    DisplayDraw { x: u8, y: u8, n: u8 },
}
