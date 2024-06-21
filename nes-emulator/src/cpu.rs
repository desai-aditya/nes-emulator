use num_derive::FromPrimitive;

pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub processor_status: u8,
}

#[derive(FromPrimitive)]
pub enum OpCode {
    LDA,
    BRK,
    TAX,
}
impl OpCode {
    pub fn from_u8(opcode: u8) -> Self {
        match opcode {
            0xa9 => OpCode::LDA,
            0x00 => OpCode::BRK,
            0xaa => OpCode::TAX,
            _ => panic!("Invalid opcode"),
        }
    }
}

pub enum AddressingMode {}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
        }
    }

    pub fn update_process_status(&mut self, result_register: u8) {
        if result_register == 0 {
            self.processor_status |= 0b0000_0010;
        } else {
            self.processor_status &= 0b1111_1101;
        }

        if result_register & 0b1000_0000 == 0b1000_0000 {
            self.processor_status |= 0b1000_0000;
        } else {
            self.processor_status &= 0b0111_1111;
        }
    }

    pub fn process_tax(&mut self) {
        self.register_x = self.accumulator;
        self.update_process_status(self.register_x);
    }

    pub fn process_lda(&mut self, program: Vec<u8>) {
        self.accumulator = program[self.program_counter as usize];
        self.program_counter += 1;
        self.update_process_status(self.accumulator);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        let opcode = program[self.program_counter as usize];
        self.program_counter += 1;

        match OpCode::from_u8(opcode) {
            OpCode::LDA => self.process_lda(program),
            OpCode::BRK => return,
            OpCode::TAX => self.process_tax(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x01]);
        assert_eq!(cpu.accumulator, 1);
    }

    #[test]
    fn test_lda_with_status_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00]);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.processor_status, 0b0000_0010);
    }

    #[test]
    fn test_lda_with_status_negative() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x80]);
        assert_eq!(cpu.accumulator, 0x80);
        assert_eq!(cpu.processor_status, 0b1000_0000);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
