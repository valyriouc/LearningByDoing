
enum Register {
    R_R0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC,
    R_COND,
}

enum InstructionSet {
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP    /* execute trap */
}

impl InstructionSet {

    fn from_u16(value: u16) -> InstructionSet {
        match value {
            0 => InstructionSet::OP_BR,
            1 => InstructionSet::OP_ADD,
            2 => InstructionSet::OP_LD,
            3 => InstructionSet::OP_ST,
            4 => InstructionSet::OP_JSR,
            5 => InstructionSet::OP_AND,
            6 => InstructionSet::OP_LDR,
            7 => InstructionSet::OP_STR,
            8 => InstructionSet::OP_RTI,
            9 => InstructionSet::OP_NOT,
            10 => InstructionSet::OP_LDI,
            11 => InstructionSet::OP_STI,
            12 => InstructionSet::OP_JMP,
            13 => InstructionSet::OP_RES,
            14 => InstructionSet::OP_LEA,
            15 => InstructionSet::OP_TRAP,
            _ => panic!("no such instruction")
        }
    }
}

enum ConditionFlag {
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2
}

fn read_image(path: &String) -> bool {

    false
}

fn read_memory(instruct: &u16, memory: &mut [u16; 65536]) -> u16 {
    memory[*instruct as usize]
}

fn sign_extend(value: u16, bit_count: i32) -> u16 {
    let mut t = value;
    if ((t >> (bit_count - 1)) & 1) == 1 {
        t |= 0xFFFF << bit_count;
    }
    t
}

fn update_flags(registers: &mut [u16; 10], r: u16) {
    if registers[r as usize] == 0 {
        registers[Register::R_COND as usize] = ConditionFlag::FL_ZRO as u16;
    }
    else if registers[r as usize] >> 15 == 1{
        registers[Register::R_COND as usize] = ConditionFlag::FL_NEG as u16;
    }
    else {
        registers[Register::R_COND as usize] = ConditionFlag::FL_POS as u16;
    }
}

fn add_operation(registers: &mut [u16; 10]) {



}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("lc3 [image-file1] ...");
        return;
    }

    for image in args[1..].into_iter() {
        if !read_image(image) {
            println!("Could not read image");
            return;
        }
    }

    let mut memory: [u16; 65536] = [0; 65536];
    let mut registers: [u16; 10] = [0; 10]; 

    registers[Register::R_COND as usize] = ConditionFlag::FL_ZRO as u16;
    registers[Register::R_PC as usize] = 0x3000;

    loop {

        let next_instr = registers[Register::R_PC as usize] + 1;
        let instr = read_memory(&next_instr, &mut memory);
        let op = InstructionSet::from_u16(instr >> 12); 

        match op {
            InstructionSet::OP_BR  => todo!(),
            InstructionSet::OP_ADD => todo!(),
            InstructionSet::OP_LD => todo!(),
            InstructionSet::OP_ST => todo!(),
            InstructionSet::OP_JSR => todo!(),
            InstructionSet::OP_AND => todo!(),
            InstructionSet::OP_LDR => todo!(),
            InstructionSet::OP_STR => todo!(),
            InstructionSet::OP_RTI => todo!(),
            InstructionSet::OP_NOT => todo!(),
            InstructionSet::OP_LDI => todo!(),
            InstructionSet::OP_STI => todo!(),
            InstructionSet::OP_JMP => todo!(),
            InstructionSet::OP_RES => todo!(),
            InstructionSet::OP_LEA => todo!(),
            InstructionSet::OP_TRAP => todo!(),
            _ => {
                println!("This operation is not implemented")
            }

        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_shifting() {
        let val: u16 = 12000;
        let es: u16 = (val >> 12); 

        assert_eq!(es + 1, 3001);
    }

    #[test]
    fn test_sign_extend() {
       let val = (-1 >> (5 - 1)) & 1;
       assert_eq!(val, 0);
    }
}