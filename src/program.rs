use crate::{hardware::memory::MemoryBus, CPU::Instruction};

/// An iterator that iterates over all of the program bytes and parses them into `Instruction`s
pub struct Program {
    /// The Program Counter (PC) or index into the program bytes
    program_counter: u16,
    /// The memory bus of the Game Boy
    // TODO: Check if this can't be a read-only reference into the ROM part of the memory bus
    bus: MemoryBus,
    /// Wether the next opcode is prefixed or not
    prefixed: bool,
}

impl Program {
    pub fn new(bus: MemoryBus) -> Self {
        Self {
            program_counter: 0,
            prefixed: false,
            bus,
        }
    }

    #[inline]
    pub fn is_prefixed(&self) -> bool {
        self.prefixed
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory 
    /// without incrementing the `program_counter`.
    pub fn read_immediate(&self) -> &u8 {
        self.bus.read8(self.program_counter)
    }

    /// Reads the current (immediate) byte, pointed to by the `program_counter` in memory, 
    /// then increments the `program_counter` by `1`.
    pub fn next_byte(&mut self) -> &u8 {
        let byte = self.bus.read8(self.program_counter);
        self.program_counter += 1;

        return byte;
    }

    /// Decodes the next instruction of the program
    ///
    /// @alias self.next()
    #[inline]
    pub fn next_instruction(&mut self) -> Option<Instruction> {
        self.next()
    }
}

impl Iterator for Program {
    type Item = Instruction;

    /// Decodes the next instruction of the program
    ///
    /// @alias self.next_instruction()
    fn next(&mut self) -> Option<Self::Item> {
        let opcode = *self.read_immediate();
        self.program_counter += 1;

        if opcode == Instruction::PREFIX_INDICATION_BYTE {
            self.prefixed = true;
            return self.next();
        }

        return match Instruction::try_from_opcode(opcode, self) {
            Ok(instruction) => Some(instruction),
            Err(msg) => panic!("{msg}"),
        };
    }
}