use std::convert::TryInto;

use crate::instruction::InstructionParam;
use crate::instruction_set::{InstructionSize, Register};

pub struct MachineState {
    pub i0: i64,
    pub i1: i64,
    pub i2: i64,
    pub i3: i64,
    pub i4: i64,
    pub i5: i64,
    pub i6: i64,
    pub i7: i64,

    pub pc: i64,
    pub sp: i64,

    pub memory: Vec<u8>,

    pub halt: bool,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            i0: 0,
            i1: 0,
            i2: 0,
            i3: 0,
            i4: 0,
            i5: 0,
            i6: 0,
            i7: 0,

            pc: 0x1000,
            sp: 0x0,
            memory: vec![0; 0x10000],

            halt: false,
        }
    }

    pub fn get_value(&self, instruction_param: &InstructionParam, size: &InstructionSize) -> i64 {
        let value = match instruction_param {
            InstructionParam::Immediate(value) => *value,
            InstructionParam::Register(register) => match register {
                Register::I0 => self.i0,
                Register::I1 => self.i1,
                Register::I2 => self.i2,
                Register::I3 => self.i3,
                Register::I4 => self.i4,
                Register::I5 => self.i5,
                Register::I6 => self.i6,
                Register::I7 => self.i7,
            },
        };
        match size {
            InstructionSize::OneByte => value as i8 as i64,
            InstructionSize::TwoByte => value as i16 as i64,
            InstructionSize::FourByte => value as i32 as i64,
            InstructionSize::EightByte => value,
        }
    }

    pub fn set_value(
        &mut self,
        value: i64,
        instruction_param: &InstructionParam,
        size: &InstructionSize,
    ) {
        let value = match size {
            InstructionSize::OneByte => value as i8 as i64,
            InstructionSize::TwoByte => value as i16 as i64,
            InstructionSize::FourByte => value as i32 as i64,
            InstructionSize::EightByte => value,
        };
        match instruction_param {
            InstructionParam::Immediate(_value) => panic!("Cannot set value on immediate argument"),
            InstructionParam::Register(register) => match register {
                Register::I0 => self.i0 = value,
                Register::I1 => self.i1 = value,
                Register::I2 => self.i2 = value,
                Register::I3 => self.i3 = value,
                Register::I4 => self.i4 = value,
                Register::I5 => self.i5 = value,
                Register::I6 => self.i6 = value,
                Register::I7 => self.i7 = value,
            },
        };
    }

    pub fn read_memory1(&self, address: i64) -> i8 {
        self.memory[address as usize] as i8
    }

    pub fn read_memory2(&self, address: i64) -> i16 {
        let value = &self.memory[address as usize..address as usize + 2];
        i16::from_le_bytes(value.try_into().unwrap())
    }

    pub fn read_memory4(&self, address: i64) -> i32 {
        let value = &self.memory[address as usize..address as usize + 4];
        i32::from_le_bytes(value.try_into().unwrap())
    }

    pub fn read_memory8(&self, address: i64) -> i64 {
        let value = &self.memory[address as usize..address as usize + 8];
        i64::from_le_bytes(value.try_into().unwrap())
    }

    pub fn write_memory1(&mut self, address: i64, value: i8) {
        self.memory[address as usize] = value as u8;
    }
    pub fn write_memory2(&mut self, address: i64, value: i16) {
        self.memory[address as usize..address as usize + 2].copy_from_slice(&value.to_le_bytes());
    }
    pub fn write_memory4(&mut self, address: i64, value: i32) {
        self.memory[address as usize..address as usize + 4].copy_from_slice(&value.to_le_bytes());
    }
    pub fn write_memory8(&mut self, address: i64, value: i64) {
        self.memory[address as usize..address as usize + 8].copy_from_slice(&value.to_le_bytes());
    }
}
