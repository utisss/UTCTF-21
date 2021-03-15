use std::num::Wrapping;
use std::process;
extern crate wasm_bindgen;

struct MemState {
    regs: [u32; 8],
    mem: [u32; 0x4000],
    end: bool,
}

const LABEL: u32 = 42069;
const LD: u32 = 1;
const ADD: u32 = 2;
const MULT: u32 = 4;
const ST: u32 = 5;
const BR: u32 = 6;
const BREQ: u32 = 7;
const MOV: u32 = 8;
const CRC32: u32 = 9;
const MOVIMM: u32 = 10;

const A: u32 = 0;
const B: u32 = 1;
const C: u32 = 2;
const D: u32 = 3;
const E: u32 = 4;
const F: u32 = 5;
const G: u32 = 6;
const PC: u32 = 7;

type Instr = fn(&mut MemState);

fn GetFunc(op: u32) -> Instr {
    match op {
        LABEL => Label,
        LD => Ld,
        ADD => Add,
        MULT => Mult,
        ST => St,
        BR => Br,
        BREQ => BrEq,
        MOV => Mov,
        CRC32 => Crc32,
        MOVIMM => MovImm,
        _ => End,
    }
}

fn End(mem: &mut MemState) {
    // println!("opcode not found");

    for i in 0..45 {
        print!("{}, ", mem.mem[0x1000 + i]);
    }
    // println!("");
    mem.end = true;
}

fn GetImm(mem: &mut MemState) -> u32 {
    let res = mem.mem[mem.regs[PC as usize] as usize];
    mem.regs[PC as usize]+=1;

    return res;
}

fn MovImm(mem: &mut MemState) {
    let reg_idx = GetImm(mem);
    let imm = GetImm(mem);
    // println!("movimm {}, {}", reg_idx, imm);
    mem.regs[reg_idx as usize] = imm;
}

fn Label(mem: &mut MemState) {
    let l = GetImm(mem);
    // println!("label {}", l);
}

fn FindLabel(mem: &mut MemState, label: u32) -> u32{
    for i in 0..mem.mem.len() {
        if mem.mem[i] == LABEL && mem.mem[i+1] == label {
            return i as u32;
        }
    }
    return 0xffffffff;
}

fn BrEq(mem: &mut MemState) {
    let a_idx = GetImm(mem);
    let b_idx = GetImm(mem);
    let label = GetImm(mem);
    if mem.regs[a_idx as usize] == mem.regs[b_idx as usize] {
        let dest = FindLabel(mem, label);
        mem.regs[PC as usize] = dest;
    }
    // println!("breq {}, {}, {}", a_idx, b_idx, label);
}

fn Br(mem: &mut MemState) {
    let label = GetImm(mem);
    let dest = FindLabel(mem, label);
    mem.regs[PC as usize] = dest;
    
    // println!("br {}", label);
}

fn Mov(mem: &mut MemState) {
    let a_idx = GetImm(mem);
    let b_idx = GetImm(mem);

    mem.regs[a_idx as usize] = mem.regs[b_idx as usize];
    // println!("mov {}, {}", a_idx, b_idx);
}

fn Add(mem: &mut MemState) {
    let a_idx = GetImm(mem);
    let b_idx = GetImm(mem);
    let val = Wrapping(mem.regs[a_idx as usize]) + Wrapping(mem.regs[b_idx as usize]);

    mem.regs[a_idx as usize] = val.0;
    // println!("add {}, {}", a_idx, b_idx);
}

fn Ld(mem: &mut MemState) {
    let dest_idx = GetImm(mem);
    let addr = mem.regs[GetImm(mem) as usize];
    mem.regs[dest_idx as usize] = mem.mem[addr as usize];
    
    // println!("ld {} from {}", dest_idx, addr);
}

fn Crc32(mem: &mut MemState) {
    let reg_idx = GetImm(mem);
    let byte_val: [u8; 1] = [mem.regs[reg_idx as usize] as u8];
    mem.regs[reg_idx as usize] = crc32(&byte_val);
    
    // println!("crc32 {}", reg_idx);
}

fn Mult(mem: &mut MemState) {
    let a_idx = GetImm(mem);
    let b_idx = GetImm(mem);
    let val = Wrapping(mem.regs[a_idx as usize]) * Wrapping(mem.regs[b_idx as usize]);

    mem.regs[a_idx as usize] = val.0;
    // println!("mult {}, {}", a_idx, b_idx);
}

fn St(mem: &mut MemState) {
    let src_idx = GetImm(mem);
    let addr = mem.regs[GetImm(mem) as usize];
    mem.mem[addr as usize] = mem.regs[src_idx as usize];

    // println!("ld {} to {}", src_idx, addr);
}

const CODE: [u32; 113] = [
    MOVIMM, A, 0x1000,
    MOVIMM, B, 0,
    MOVIMM, C, 40,
    LABEL, 0,
        BREQ, B, C, 1,
        
        MOV, D, B,
        ADD, D, A,
        
        LD, E, D,
        CRC32, E,
        MOVIMM, F, 0x7fffffff,
        MULT, E, F, 
        MOVIMM, F, 8753,
        ADD, E, F,
        ST, E, D,

        MOVIMM, D, 1, 
        ADD, B, D,
        BR, 0,
    LABEL, 1,
    MOVIMM, B, 0,
    LABEL, 2,
        BREQ, B, C, 5,

        MOVIMM, D, 0,
        LABEL, 3,
            BREQ, B, D, 4,
            
            MOV, E, B, 
            ADD, E, A,
            LD, F, E,
            
            MOV, E, D,
            ADD, E, A,
            LD, G, E,
            
            ADD, F, G,

            ST, F, E, 

            MOVIMM, E, 1,
            ADD, D, E,
            BR, 3,
        LABEL, 4,
        MOVIMM, D, 1,
        ADD, B, D,
        BR, 2,
    LABEL, 5,
];

fn crc32_compute_table() -> [u32; 256] {
    let mut crc32_table = [0; 256];
 
    for n in 0..256 {
        crc32_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
            match acc & 1 {
                1 => 0xedb88320 ^ (acc >> 1),
                _ => acc >> 1,
            }
        });
    }
 
    crc32_table
}
 
fn crc32(buf: &[u8]) -> u32 {
    let crc_table = crc32_compute_table();
 
    !buf.iter().fold(!0, |acc, octet| {
        (acc >> 8) ^ crc_table[((acc & 0xff) ^ *octet as u32) as usize]
    })
}

const FLAG: [u32; 40] = [3376432092, 3148712937, 1092076640, 3085618703, 1307668188, 3064531694, 3095200819, 1314007355, 1606225393, 1858895620, 2335139813, 3037063580, 1259113065, 3219670873, 938526970, 969196095, 1671119862, 4272550096, 269800838, 522471065, 294751910, 2811768691, 1081432078, 1783355845, 4021686844, 18937586, 4043001627, 1741576394, 4258593175, 1720489385, 1751158510, 2453082277, 2213928557, 1943025302, 2644949069, 866998554, 347023697, 2340565760, 4097429266, 55132197]; //utflag{how_l0ng_should_theflag_be_lmfao}

#[wasm_bindgen]
fn check_flag(flag: &[u8]) -> bool {
    if(flag.len() != 40) {
        return false;
    }

    let mut state = MemState{
        regs: [0; 8], 
        mem: [0; 0x4000], 
        end: false
    };

    for i in 0..flag.len() {
        state.mem[0x1000 + i] = flag[i].into();
    }

    for i in 0..CODE.len() {
        state.mem[i] = CODE[i];
    }

    while !state.end {
        // println!("{:?}", state.regs);
        let instr = GetFunc(GetImm(&mut state));
        instr(&mut state);
    }

    for i in 0..40 {
        if(state.mem[0x1000 + i] != FLAG[i]) {
            return false;
        }
    }

    return true;
}

// fn main() {
//     // println!("{}", check_flag("utflag{how_l0ng_should_theflag_be_lmfao}".as_bytes()));
// }