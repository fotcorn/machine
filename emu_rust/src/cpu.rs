use crate::instruction::Instruction;
use crate::instruction_set::InstructionType;
use crate::machine_state::MachineState;

pub fn execute(machine_state: &mut MachineState, instruction: &Instruction) {
    match instruction.instruction {
        InstructionType::MOV => {
            assert!(
                instruction.params.len() == 2,
                "MOV instruction requires two arguments"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            machine_state.set_value(value, &instruction.params[1], &instruction.size);
        }
        InstructionType::LD => panic!("Not implemented instruction LD"),
        InstructionType::STR => panic!("Not implemented instruction STR"),

        // arithmetic
        InstructionType::ADD => panic!("Not implemented instruction ADD"),
        InstructionType::SUB => panic!("Not implemented instruction SUB"),
        InstructionType::INC => panic!("Not implemented instruction INC"),
        InstructionType::DEC => panic!("Not implemented instruction DEC"),
        InstructionType::DIV => panic!("Not implemented instruction DIV"),
        InstructionType::MUL => panic!("Not implemented instruction MUL"),
        InstructionType::MOD => panic!("Not implemented instruction MOD"),

        // binary
        InstructionType::AND => panic!("Not implemented instruction AND"),
        InstructionType::OR => panic!("Not implemented instruction OR"),
        InstructionType::XOR => panic!("Not implemented instruction XOR"),
        InstructionType::NOT => panic!("Not implemented instruction NOT"),

        // compare & jumps
        InstructionType::JMP => panic!("Not implemented instruction JMP"),
        InstructionType::JE => panic!("Not implemented instruction JE"),
        InstructionType::JNE => panic!("Not implemented instruction JNE"),

        // conditional jumps
        InstructionType::JL => panic!("Not implemented instruction JL"),
        InstructionType::JLE => panic!("Not implemented instruction JLE"),
        InstructionType::JG => panic!("Not implemented instruction JG"),
        InstructionType::JGE => panic!("Not implemented instruction JGE"),

        // stack
        InstructionType::PUSH => panic!("Not implemented instruction PUSH"),
        InstructionType::POP => panic!("Not implemented instruction POP"),
        InstructionType::CALL => panic!("Not implemented instruction CALL"),
        InstructionType::RET => panic!("Not implemented instruction RET"),

        // io
        InstructionType::IN => panic!("Not implemented instruction IN"),
        InstructionType::OUT => panic!("Not implemented instruction OUT"),

        // syscalls
        InstructionType::SYSCALL => panic!("Not implemented instruction SYSCALL"),
        InstructionType::SYSRET => panic!("Not implemented instruction SYSRET"),

        // other
        InstructionType::HALT => panic!("Not implemented instruction HALT"),
    }
}
