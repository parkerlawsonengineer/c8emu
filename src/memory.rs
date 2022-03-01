use std::fs::File;
use std::io::Read;

pub struct Memory {
    mem: Vec<u8>
}


impl Memory {

    pub fn new() -> Self {
        Self {
            mem: vec![0; 4096]
        }
    }

    pub fn initialize(&mut self, filename: &String) { 

        //initialize the array to all zeroes
        self.mem = vec![0; 4096];

        //set first 80 bytes of memory to font files
        self.init_font();

        let mut file = File::open(filename).expect("Error opening file.");
        let mut buf: [u8; 0xE00] = [0;0xE00]; //create buffer of max file size (total memory size 4096 - system files 512 == 3584 == 0xE00

        let num_bytes = file.read(&mut buf).expect("error reading from file.");

        for n in 0..num_bytes {
            self.mem[0x200 + n] = buf[n];
        }

    }

    fn init_font(&mut self) { //loads system font into memory, this is expected by all chip-8 ROMS

        //0
        self.mem[0] = 0xF0;
        self.mem[1] = 0x90;
        self.mem[2] = 0x90;
        self.mem[3] = 0x90;
        self.mem[4] = 0xF0;

        //1
        self.mem[5] = 0x20;
        self.mem[6] = 0x60;
        self.mem[7] = 0x20;
        self.mem[8] = 0x20;
        self.mem[9] = 0x70;

        //2
        self.mem[10] = 0xF0;
        self.mem[11] = 0x10;
        self.mem[12] = 0xF0;
        self.mem[13] = 0x80;
        self.mem[14] = 0xF0;

        //3
        self.mem[15] = 0xF0;
        self.mem[16] = 0x10;
        self.mem[17] = 0xF0;
        self.mem[18] = 0x80;
        self.mem[19] = 0xF0;

        //4
        self.mem[20] = 0x90;
        self.mem[21] = 0x90;
        self.mem[22] = 0xF0;
        self.mem[23] = 0x10;
        self.mem[24] = 0x10;

        //5
        self.mem[25] = 0xF0;
        self.mem[26] = 0x80;
        self.mem[27] = 0xF0;
        self.mem[28] = 0x10;
        self.mem[29] = 0xF0;

        //6
        self.mem[30] = 0xF0;
        self.mem[31] = 0x80;
        self.mem[32] = 0xF0;
        self.mem[33] = 0x90;
        self.mem[34] = 0xF0;

        //7
        self.mem[35] = 0xF0;
        self.mem[36] = 0x10;
        self.mem[37] = 0x20;
        self.mem[38] = 0x40;
        self.mem[39] = 0x40;

        //8
        self.mem[40] = 0xF0;
        self.mem[41] = 0x90;
        self.mem[42] = 0xF0;
        self.mem[43] = 0x90;
        self.mem[44] = 0xF0;

        //9
        self.mem[45] = 0xF0;
        self.mem[46] = 0x90;
        self.mem[47] = 0xF0;
        self.mem[48] = 0x10;
        self.mem[49] = 0xF0;

        //A
        self.mem[50] = 0xF0;
        self.mem[51] = 0x90;
        self.mem[52] = 0xF0;
        self.mem[53] = 0x90;
        self.mem[54] = 0x90;

        //B
        self.mem[55] = 0xE0;
        self.mem[56] = 0x90;
        self.mem[57] = 0xE0;
        self.mem[58] = 0x90;
        self.mem[59] = 0xE0;

        //C
        self.mem[60] = 0xF0;
        self.mem[61] = 0x80;
        self.mem[62] = 0x80;
        self.mem[63] = 0x80;
        self.mem[64] = 0xF0;

        //D
        self.mem[65] = 0xE0;
        self.mem[66] = 0x90;
        self.mem[67] = 0x90;
        self.mem[68] = 0x90;
        self.mem[69] = 0xE0;

        //E
        self.mem[70] = 0xF0;
        self.mem[71] = 0x80;
        self.mem[72] = 0xF0;
        self.mem[73] = 0x80;
        self.mem[74] = 0xF0;

        //F
        self.mem[75] = 0xF0;
        self.mem[76] = 0x80;
        self.mem[77] = 0xF0;
        self.mem[78] = 0x80;
        self.mem[79] = 0x80

    }

    pub fn set_memory(&mut self, address: u16, val: u8) {
        self.mem[address as usize] = val;
    }

    pub fn get_memory(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

}
