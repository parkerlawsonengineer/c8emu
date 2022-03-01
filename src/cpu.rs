use crate::memory::Memory;
use crate::display_module::DisplayModule;
use rand::Rng;
use crate::timer_module::TimerModule;
use crate::keyboard_module::KeyboardModule;


pub struct Cpu {

    //registers
    pc: u16,
    i: u16,
    sp: u8,
    stack: [u16; 16],
    reg: [u8; 16],
    wait: bool,
    key_reg: u8
}

impl Cpu {

    pub fn new() -> Self {
        Self{
            pc: 0x200,
            i: 0,
            sp: 0,
            stack: [0;16],
            reg: [0;16],
            wait: false,
            key_reg: 0
        }
    }

    pub fn execute_instruction(&mut self, memory: &mut Memory, display_module: &mut DisplayModule, timer_module: &mut TimerModule, keyboard_module: &mut KeyboardModule) {

        if !self.wait {
            print!("{:X}", self.pc);

            //load instruction from memory (memory[pc] in first byte, memory[pc + 1] in second byte)
            let instr: u16 = (memory.get_memory(self.pc) as u16) << 8 |
                memory.get_memory(self.pc + 1) as u16;

            print!(" {:X} ", instr);

            self.pc += 2; //increment PC here so that jump functions work

            if instr == 0x00E0 {                        //00E0 - CLS
                println!("CLS");
                display_module.clear(); //clear display buffer
            } else if instr == 0x00EE {                 //00EE - RET
                println!("RET");
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1; //sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            } else if instr & 0xF000 == 0x1000 {        //1nnn - JP addr
                let jp_addr = self.get_nnn(instr);
                println!("JP {:X}", jp_addr);
                self.pc = jp_addr;
            } else if instr & 0xF000 == 0x2000 {        //2nnn - CALL addr
                let call_addr = self.get_nnn(instr);
                println!("CALL {:X}", call_addr);
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = call_addr; //increment stack pointer, add current PC to stack, then set PC to addr
            } else if instr & 0xF000 == 0x3000 {        //3xkk - SE Vx, byte
                let se_x = self.get_nibble(instr, 1) as usize;
                let se_kk = self.get_kk(instr);
                println!("SE V{:X}, {:X}", se_x, se_kk);
                if self.reg[se_x] == se_kk {
                    self.pc += 2; //if register Vx matches kk, skip next instruction
                }
            } else if instr & 0xF000 == 0x4000 {        //4xkk - SNE Vx, byte
                let sne_x = self.get_nibble(instr, 1) as usize;
                let sne_kk = self.get_kk(instr);
                println!("SNE V{:X}, {:X}", sne_x, sne_kk);
                if self.reg[sne_x] != sne_kk {
                    self.pc += 2; //if register Vx does not match kk, skip next instruction
                }
            } else if instr & 0xF00F == 0x5000 {        //5xy0 - SE Vx, Vy
                let se_x = self.get_nibble(instr, 1) as usize;
                let se_y = self.get_nibble(instr, 2) as usize;
                println!("SE V{:X}, V{:X}", se_x, se_y);
                if self.reg[se_x] == self.reg[se_y] {
                    self.pc += 2; //if register Vx matches register Vy, skip next instruction
                }
            } else if instr & 0xF000 == 0x6000 {        //6xkk - LD Vx, byte
                let ld_x = self.get_nibble(instr, 1) as usize;
                let ld_kk = self.get_kk(instr);
                println!("LD V{:X}, {:X}", ld_x, ld_kk);
                self.reg[ld_x] = ld_kk; //load kk into Vx register
            } else if instr & 0xF000 == 0x7000 {        //7xkk - ADD Vx, byte
                let add_x = self.get_nibble(instr, 1) as usize;
                let add_kk = self.get_kk(instr);
                println!("ADD V{:X}, {:X}", add_x, add_kk);
                self.reg[add_x] = ((self.reg[add_x] as usize + add_kk as usize) % 256) as u8; //add kk to Vx register, mod 256 to prevent overflow
            } else if instr & 0xF00F == 0x8000 {        //8xy0 - LD Vx, Vy
                let ld_x = self.get_nibble(instr, 1) as usize;
                let ld_y = self.get_nibble(instr, 2) as usize;
                println!("LD V{:X}, V{:X}", ld_x, ld_y);
                self.reg[ld_x] = self.reg[ld_y]; //load value from Vy into Vx
            } else if instr & 0xF00F == 0x8001 {        //8xy1 - OR Vx, Vy
                let or_x = self.get_nibble(instr, 1) as usize;
                let or_y = self.get_nibble(instr, 2) as usize;
                println!("OR V{:X}, V{:X}", or_x, or_y);
                self.reg[or_x] = self.reg[or_x] | self.reg[or_y]; //bitwise-or Vx and Vy, store in Vx
            } else if instr & 0xF00F == 0x8002 {        //8xy2 - AND Vx, Vy
                let and_x = self.get_nibble(instr, 1) as usize;
                let and_y = self.get_nibble(instr, 2) as usize;
                println!("AND V{:X}, V{:X}", and_x, and_y);
                self.reg[and_x] = self.reg[and_x] & self.reg[and_y]; //bitwise-and Vx and Vy, store in Vx
            } else if instr & 0xF00F == 0x8003 {        //8xy3 - XOR Vx, Vy
                let xor_x = self.get_nibble(instr, 1) as usize;
                let xor_y = self.get_nibble(instr, 2) as usize;
                println!("XOR V{:X}, V{:X}", xor_x, xor_y);
                self.reg[xor_x] = self.reg[xor_x] ^ self.reg[xor_y]; //bitwise-xor Vx and Vy, store in Vx
            } else if instr & 0xF00F == 0x8004 {        //8xy4 - ADD Vx, Vy
                let add_x = self.get_nibble(instr, 1) as usize;
                let add_y = self.get_nibble(instr, 2) as usize;
                println!("ADD V{:X}, V{:X}", add_x, add_y);
                let result: u16 = (self.reg[add_x] as u16) + (self.reg[add_y] as u16);
                if result > 0xFF {
                    self.reg[0xF] = 1;
                }
                self.reg[add_x] = (result & 0x00FF) as u8; //add Vx and Vy, store in Vx. if result would overflow, set VF to 1
            } else if instr & 0xF00F == 0x8005 {        //8xy5 - SUB Vx, Vy
                let sub_x = self.get_nibble(instr, 1) as usize;
                let sub_y = self.get_nibble(instr, 2) as usize;
                println!("SUB V{:X}, V{:X}", sub_x, sub_y);
                self.reg[0xF] = if self.reg[sub_x] > self.reg[sub_y] { 1 } else { 0 };
                self.reg[sub_x] = (self.reg[sub_x] as i16 - self.reg[sub_y] as i16) as u8; //sub Vx and Vy, if result would be negative, set VF to 1
            } else if instr & 0xF00F == 0x8006 {        //8xy6 - SHR Vx {, Vy}
                let shr_x = self.get_nibble(instr, 1) as usize;
                println!("SHR V:{:X}", shr_x);
                if self.reg[shr_x] & 0x0001 == 1 {
                    self.reg[0xF] = 1;
                } else {
                    self.reg[0xF] = 0;
                }
                self.reg[shr_x] = self.reg[shr_x] >> 1;
            } else if instr & 0xF00F == 0x8007 {        //8xy7 - SUBN Vx, Vy
                let subn_x = self.get_nibble(instr, 1) as usize;
                let subn_y = self.get_nibble(instr, 1) as usize;
                println!("SUBN V{:X}, V{:X}", subn_x, subn_y);
                self.reg[0xF] = if self.reg[subn_y] > self.reg[subn_x] { 1 } else { 0 };
                let subn_val = (self.reg[subn_y] as u16) - (self.reg[subn_x] as u16);
                let subn_val = subn_val & 0x0000FFFF;
                let subn_val = subn_val as u8;
                self.reg[subn_x] = subn_val;
            } else if instr & 0xF00F == 0x800E {        //8xyE - SHL Vx {, Vy}
                let shl_x = self.get_nibble(instr, 1) as usize;
                println!("SHL V{:X}", shl_x);
                self.reg[0xF] = if (self.reg[shl_x] & 0x80) >> 7 == 1 { 1 } else { 0 };
                self.reg[shl_x] = self.reg[shl_x] << 1;
            } else if instr & 0xF00F == 0x9000 { //9xy0 - SNE Vx, Vy
                let sne_x = self.get_nibble(instr, 1) as usize;
                let sne_y = self.get_nibble(instr, 2) as usize;
                println!("SNE V{:X}, V{:X}", sne_x, sne_y);
                if self.reg[sne_x] != self.reg[sne_y] {
                    self.pc += 2;
                }
            } else if instr & 0xF000 == 0xA000 {        //Annn - LD, I , addr
                let ld_i_nnn = self.get_nnn(instr);
                println!("LD I, {:X}", ld_i_nnn);
                self.i = ld_i_nnn; //set I register to nnn
            } else if instr & 0xF000 == 0xB000 {        //Bnnn - JP V0, addr
                let ld_nnn = self.get_nnn(instr);
                println!("JP V0, {:X}", ld_nnn);
                self.pc = self.reg[0] as u16 + ld_nnn //set pc to nnn + V0
            } else if instr & 0xF000 == 0xC000 {        //RND Vx, byte
                let rnd_x = self.get_nibble(instr, 1) as usize;
                let rnd_kk = self.get_kk(instr);
                println!("RND V{:X}, {:X}", rnd_x, rnd_kk);
                let rnd_num: u8 = rand::thread_rng().gen();
                self.reg[rnd_x] = rnd_num & rnd_kk; //generate random number 0-255, bitwise-and with kk
            } else if instr & 0xF000 == 0xD000 {        //Dxyn - DRW Vx, Vy, nibble
                let drw_x = self.get_nibble(instr, 1) as usize;
                let drw_y = self.get_nibble(instr, 2) as usize;
                let drw_n = self.get_nibble(instr, 3);
                println!("DRW V{:X}, V{:X}, {:X}", drw_x, drw_y, drw_n);
                let mut collision = false;
                for n in 0..drw_n {
                    if display_module.draw_sprite(self.reg[drw_x], self.reg[drw_y] + n, self.i + n as u16, memory) {
                        collision = true; //draw each byte, draw function returns whether there is a collision. if any byte collides, set VF true
                    }
                }
                self.reg[0xF] = if collision { 1 } else { 0 };
            } else if instr & 0xF0FF == 0xE09E { //Ex9E - SKP Vx
                let skp_x = self.get_nibble(instr, 1) as usize;
                if keyboard_module.get_key(self.reg[skp_x]) == true {
                    self.pc += 2; //if keyboard key associated with Vx is pressed, skip next instruction
                }
            } else if instr & 0xF0FF == 0xE0A1 {          //ExA1 - SKNP Vx
                let sknp_x = self.get_nibble(instr, 1) as usize;
                println!("SKNP V{:X}", sknp_x);
                if keyboard_module.get_key(self.reg[sknp_x]) == false { //if key is not pressed, add 2 to pc
                    self.pc += 2;
                }
            } else if instr & 0xF0FF == 0xF007 {            //Fx07 - LD Vx, DT
                let ld_x = self.get_nibble(instr, 1) as usize;
                println!("LD V{:X}, DT", ld_x);
                self.reg[ld_x] = timer_module.get_delay_register();
            } else if instr & 0xF0FF == 0xF00A {        //Fx0A - LD Vx, K
                let ld_x = self.get_nibble(instr, 1);
                println!("LD V{:X}, K", ld_x);
                self.wait = true; //stop cpu execution until a key is pressed, load key into Vx
                self.key_reg = ld_x;
            } else if instr & 0xF0FF == 0xF015 {        //Fx15 - LD DT, Vx
                let ld_dt_x = self.get_nibble(instr, 1) as usize;
                println!("LD DT, V{:X}", ld_dt_x);
                timer_module.set_delay_register(self.reg[ld_dt_x]); //load Vx into Delay Timer
            } else if instr & 0xF0FF == 0xF018 {        //Fx18 - LD ST, Vx
                let ld_st_x = self.get_nibble(instr, 1) as usize;
                println!("LD ST, V{:X}", ld_st_x);
                timer_module.set_sound_register(self.reg[ld_st_x]);
            } else if instr & 0xF0FF == 0xF01E {        //Fx1E - ADD I, Vx
                let add_i_x = self.get_nibble(instr, 1) as usize;
                println!("ADD I, V{:X}", add_i_x);
                self.i = (self.i + self.reg[add_i_x] as u16) % 65535;
            } else if instr & 0xF0FF == 0xF029 {        //Fx29 - LD F, Vx
                let ld_x = self.get_nibble(instr, 1) as usize;
                println!("LD F, V{:X}", ld_x);
                self.i = ld_x as u16 * 5;
            } else if instr & 0xF0FF == 0xF033 {          //Fx33 - LD B, Vx
                let ld_x = self.get_nibble(instr, 1) as usize;
                println!("LD B, V{:X}", ld_x);
                let vx = self.reg[ld_x];
                memory.set_memory(self.i, vx / 100);
                memory.set_memory(self.i + 1, (vx % 100) / 10);
                memory.set_memory(self.i + 2, vx % 10);
            } else if instr & 0xF0FF == 0xF055 {        //Fx55 - LD [I], Vx
                let ld_x = self.get_nibble(instr, 1) as usize;
                println!("LD [I], V{:X}", ld_x);
                for n in 0..(ld_x + 1) {
                    memory.set_memory(self.i + n as u16, self.reg[n]);
                }
            } else if instr & 0xF0FF == 0xF065 {          //Fx65 - LD Vx, [I]
                let ld_x  = self.get_nibble(instr, 1) as usize;
                println!("LD V{:X}, [I]", ld_x);
                for n in 0..(ld_x + 1) {
                    self.reg[n] = memory.get_memory(self.i + n as u16);
                }
            } else {
                println!("INSTRUCTION NOT SUPPORTED.");
            }
        }

    }

    fn get_nnn(&self, instr: u16) -> u16 {
        instr & 0x0FFF //mask off first nibble to get argument for nnn instructions
    }
    fn get_kk(&self, instr: u16) -> u8 {
        (instr & 0x00FF) as u8 //mask off first byte, return second
    }
    fn get_nibble(&self, instr: u16, nibble_num: u8) -> u8 { //0000111122223333 for nibble addressing
        let shift_amt = 12 - (4 * nibble_num);
        ((instr >> shift_amt) & 0x000F) as u8
    }

    pub fn get_wait(&self) -> bool { self.wait }

    pub fn key_press(&mut self, val: u8) {
        self.wait = false;
        self.reg[self.key_reg as usize] = val;
        println!("---------------------------V{:X}: {:X}", self.key_reg, val);
    }

}
