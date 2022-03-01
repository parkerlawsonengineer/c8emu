use crate::memory::Memory;
use std::convert::TryInto;

const STACK_SIZE: usize = 2;

pub struct DisplayModule {
    frame_buffer: Vec<Vec<bool>>, //64x32 array of bools for black and white frames
    frame_stack: Vec<Vec<Vec<bool>>>,
    width: u32,
    height: u32
}

impl DisplayModule {
    ///Initialize and Return a new DisplayUnit
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame_buffer: vec![ vec![false; 32]; 64],
            frame_stack: vec![vec![ vec![false; 32]; 64]; STACK_SIZE],
            width,
            height
        }
    }

    pub fn draw_sprite(&mut self, in_x: u8, in_y: u8, i: u16, memory: &Memory) -> bool {

        let sprite: u8 = memory.get_memory(i);

        let mut collision = false;

        for n in 0..8 {

            let x = ((in_x as usize + n) % self.width as usize) as usize;
            let y = (in_y % self.height as u8) as usize;

            let sprite_pixel = ((sprite >> (7 - n)) & 1) != 0;

            self.frame_buffer[x][y] ^= sprite_pixel;

            if sprite_pixel && !self.frame_buffer[x][y] {
                collision = true;
            }

        }

        collision
    }

    //clears buffer (set all to 0)
    pub fn clear(&mut self) {
        self.frame_buffer = vec![ vec![false; 32]; 64];
    }

    /// Draw the frame_buffer array contents to the actual frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    ///

    pub fn draw(&mut self, frame: &mut [u8]) {

        //draw current frame
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            //get coordinates
            let x = i % self.width as usize;
            let y = i / self.width as usize;

            //set pixel to white if pixel should be displayed, transparent white if it was displayed last frame, otherwise set to black
            let rgba = self.generate_glow(x,y);

            //add pixel to buffer
            pixel.copy_from_slice(&rgba);


        }

        //save current frame to previous frame
        self.frame_stack.push(self.frame_buffer.to_vec());
        self.frame_stack.remove(0);

    }

    fn generate_glow(&self, x: usize, y: usize) -> [u8;4] { //helps reduce 'stutter' in graphics by loosly emulating chip-8 era hardware

        let stack_len = self.frame_stack.len();

        for i in 0..stack_len {
            if self.frame_stack[i][x][y] {
                let pixel_opacity = (0xFF / (stack_len - i)) / 2;
                let pixel_opacity:u8 = pixel_opacity as u8;
                return [0xFF,0xFF,0xFF, pixel_opacity];
            }
        }

        [0x00,0x00,0x00,0x00]
    }



}
