pub(super) struct Register {
    /// 16-bit index register called "I" which is used to point at locations in memory
    i: u16,
    /// A program counter, often called just "PC", which points to the current instruction in memory
    program_counter: u16,
    /// A stack for 16-bit addresses which is used to call subroutines/function and return from them
    stack: Vec<u16>,
    /// 16 8-bit general purpose variable registers numbered `0` through `F`, called `V0` through `VF`
    v: [u8; 16],
}

impl Register {
    pub(super) fn new() -> Register {
        Register {
            i: 0,
            program_counter: 0x200,
            stack: Vec::new(),
            v: [0; 16],
        }
    }

    pub(super) fn get_i(&self) -> u16 {
        self.i
    }

    pub(super) fn set_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    pub(super) fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub(super) fn set_program_counter(&mut self, program_counter: u16) {
        self.program_counter = program_counter;
    }

    pub(super) fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    pub(super) fn push_stack(&mut self, nnn: u16) {
        self.stack.push(nnn);
    }

    pub(super) fn pop_stack(&mut self) -> u16 {
        self.stack.pop().expect("failed to pop stack")
    }

    pub(super) fn get_v(&self, x: u8) -> u8 {
        self.v[x as usize]
    }

    pub(super) fn set_v(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = nn;
    }

    #[cfg(feature = "editor")]
    pub(super) fn get_stack(&self) -> &[u16] {
        &self.stack
    }
}
