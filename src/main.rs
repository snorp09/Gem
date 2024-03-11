use std::io;
use std::io::Read;

#[derive(PartialEq)]
enum Flags{
    None,
    NotEq,
}


#[allow(non_camel_case_types)]
struct CPU {
    r: [u8; 8],
    pc: u16,
    flag: Flags
}


/*
Instructions:
0x00 => NO OP.
0x01 => MOV src, dst
0x02 => MOV value, dst
0x03 => ADD src, dst
0x04 => JMP address # The address is found by summing the next two bytes of data.
 */
fn do_instruction(instruct: [u8; 8], cpu: &mut CPU) {
    match instruct[0] {
        0x00 => {}
        0x01 => cpu.r[instruct[2] as usize] = cpu.r[instruct[1] as usize],
        0x02 => cpu.r[instruct[2] as usize] = instruct[1],
        0x03 => cpu.r[instruct[2] as usize] += instruct[1],
        0x04 => {
            let addr = (instruct[1] + instruct[2]) as u16;
            // Mem size is hard limited at 2048 currently.
            if addr % 8 != 0 || addr > 2048 {
                panic!("Jump called to invalid position.");
            }
            cpu.pc = addr - 8;
        }
        _ => panic!("Invalid opcode."),
    }
    cpu.pc += 8;
}

fn prgm_loop(prgm: [u8; 2048], cpu: &mut CPU){
    loop {
        if(cpu.pc >= 2048) {
            break
        }
        let instruction: [u8;8] = prgm[cpu.pc as usize..(cpu.pc+8) as usize].try_into().unwrap();
        do_instruction(instruction, cpu)
    }
}

fn main() -> io::Result<()>{
    let mut cpu = CPU {
        r: [0; 8],
        pc: 0,
        flag: Flags::None
    };
    let mut prgm_buff: [u8; 2048] = [0;2048];
    let mut file = std::fs::File::open("test.bin")?;
    file.read(&mut prgm_buff)?;
    prgm_loop(prgm_buff, &mut cpu);
    Ok(())
}
