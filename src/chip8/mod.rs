mod cpu;
mod display;
pub(crate) mod emulator;
mod font;
pub(crate) mod keypad;
mod memory;
mod operation;
mod register;
mod timer;

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::{operation::*, *};

    #[test]
    fn get_pixel() {
        let emulator = emulator::Emulator::new();

        assert!(!emulator.is_pixel_on(0, 0));
        assert!(!emulator.is_pixel_on(63, 0));
        assert!(!emulator.is_pixel_on(0, 0));
        assert!(!emulator.is_pixel_on(0, 31));
    }

    #[test]
    fn loads_rom() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut emulator = emulator::Emulator::new();
        emulator.load_rom(&rom).unwrap();
    }

    #[allow(clippy::cast_possible_truncation)]
    #[test]
    fn parse_instruction() {
        let mut rom = Vec::new();
        let mut file = std::fs::File::open("./roms/IBM Logo.ch8").unwrap();
        file.read_to_end(&mut rom).unwrap();

        let mut memory = memory::Memory::new();
        memory.load_rom(&rom).unwrap();

        let instructions = vec![
            // 00000000
            Operation::CLS(CLS::new()),
            Operation::LDI(LDI::new(0x22A)),
            Operation::LD1(LD1::new(0x0, 0x0C)),
            Operation::LD1(LD1::new(0x1, 0x08)),
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            Operation::ADD1(ADD1::new(0x0, 0x09)),
            Operation::LDI(LDI::new(0x239)),
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            // 00000010
            Operation::LDI(LDI::new(0x248)),
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            Operation::ADD1(ADD1::new(0x0, 0x04)),
            Operation::LDI(LDI::new(0x257)),
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::LDI(LDI::new(0x266)),
            // 00000020
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            Operation::ADD1(ADD1::new(0x0, 0x08)),
            Operation::LDI(LDI::new(0x275)),
            Operation::DRW(DRW::new(0x0, 0x1, 0xF)),
            Operation::JP(JP::new(0x228)),
        ];

        for (index, instruction) in instructions.into_iter().enumerate() {
            let byte1 = memory.get_byte((0x200 + index * 2) as u16);
            let byte2 = memory.get_byte((0x200 + index * 2 + 1) as u16);

            assert_eq!(
                Operation::parse([byte1, byte2]),
                instruction,
                "instruction {}",
                index
            );
        }
    }
}
