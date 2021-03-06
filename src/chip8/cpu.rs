use super::{
    display::Display, keypad::Keypad, memory::Memory, operation::Operation, register::Register,
    timer::Timer,
};

pub(super) struct Cpu;

impl Cpu {
    pub(super) fn new() -> Cpu {
        Cpu
    }

    #[allow(clippy::unused_self)]
    pub(super) fn execute(
        &self,
        register: &mut Register,
        display: &mut Display,
        keypad: &Keypad,
        memory: &mut Memory,
        delay_timer: &mut Timer,
        sound_timer: &mut Timer,
    ) {
        let instruction = Operation::parse([
            memory.get_byte(register.get_program_counter()),
            memory.get_byte(register.get_program_counter() + 0x1),
        ]);

        match instruction {
            Operation::CLS(o) => o.execute(register, display),
            Operation::RET(o) => o.execute(register),
            Operation::SYS(o) => o.execute(),
            Operation::JP(o) => o.execute(register),
            Operation::CALL(o) => o.execute(register),
            Operation::SE1(o) => o.execute(register),
            Operation::SNE1(o) => o.execute(register),
            Operation::SE2(o) => o.execute(register),
            Operation::LD1(o) => o.execute(register),
            Operation::ADD1(o) => o.execute(register),
            Operation::LD2(o) => o.execute(register),
            Operation::OR(o) => o.execute(register),
            Operation::AND2(o) => o.execute(register),
            Operation::XOR(o) => o.execute(register),
            Operation::ADD2(o) => o.execute(register),
            Operation::SUB(o) => o.execute(register),
            Operation::SHR(o) => o.execute(register),
            Operation::SUBN(o) => o.execute(register),
            Operation::SHL(o) => o.execute(register),
            Operation::SNE2(o) => o.execute(register),
            Operation::LDI(o) => o.execute(register),
            Operation::JPV0(o) => o.execute(register),
            Operation::RND(o) => o.execute(register),
            Operation::DRW(o) => o.execute(register, display, memory),
            Operation::SKP(o) => o.execute(register, keypad),
            Operation::SKNP(o) => o.execute(register, keypad),
            Operation::LDVDT(o) => o.execute(register, delay_timer),
            Operation::LDK(o) => o.execute(register, keypad),
            Operation::LDDTV(o) => o.execute(register, delay_timer),
            Operation::LDST(o) => o.execute(register, sound_timer),
            Operation::ADDI(o) => o.execute(register),
            Operation::LDF(o) => o.execute(register),
            Operation::LDB(o) => o.execute(register, memory),
            Operation::LDIV(o) => o.execute(register, memory),
            Operation::LDVI(o) => o.execute(register, memory),
        }
    }
}
