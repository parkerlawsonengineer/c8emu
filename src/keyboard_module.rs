use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;
use crate::cpu::Cpu;

pub struct KeyboardModule {
    keys: [bool; 16]
}

impl KeyboardModule {
    pub fn new() -> Self {
        KeyboardModule {
            keys: [false; 16]
        }
    }

    pub fn set_key(&mut self, key: u8, val: bool) {
        self.keys[key as usize] = val;
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    fn check_single_key(&mut self, input: &WinitInputHelper, cpu: &mut Cpu, key: VirtualKeyCode, key_addr: u8) {

        if input.key_pressed(key) {
            self.set_key(key_addr, true);
            if cpu.get_wait() {
                cpu.key_press(key_addr);
            }
        } else if input.key_held(key) {
            self.set_key(key_addr, true);
        } else if input.key_released(key) {
            self.set_key(key_addr, false);
        }

    }

    pub fn check_keys(&mut self, input: &WinitInputHelper, cpu: &mut Cpu) {

        //regular keyboard
        self.check_single_key(input, cpu, VirtualKeyCode::Key0, 0x0);
        self.check_single_key(input, cpu, VirtualKeyCode::Key1, 0x1);
        self.check_single_key(input, cpu, VirtualKeyCode::Key2, 0x2);
        self.check_single_key(input, cpu, VirtualKeyCode::Key3, 0x3);
        self.check_single_key(input, cpu, VirtualKeyCode::Key4, 0x4);
        self.check_single_key(input, cpu, VirtualKeyCode::Key5, 0x5);
        self.check_single_key(input, cpu, VirtualKeyCode::Key6, 0x6);
        self.check_single_key(input, cpu, VirtualKeyCode::Key7, 0x7);
        self.check_single_key(input, cpu, VirtualKeyCode::Key8, 0x8);
        self.check_single_key(input, cpu, VirtualKeyCode::Key9, 0x9);
        self.check_single_key(input, cpu, VirtualKeyCode::A, 0xA);
        self.check_single_key(input, cpu, VirtualKeyCode::B, 0xB);
        self.check_single_key(input, cpu, VirtualKeyCode::C, 0xC);
        self.check_single_key(input, cpu, VirtualKeyCode::D, 0xD);
        self.check_single_key(input, cpu, VirtualKeyCode::E, 0xE);
        self.check_single_key(input, cpu, VirtualKeyCode::F, 0xF);

        //numpad
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad0, 0x0);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad1, 0x1);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad2, 0x2);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad3, 0x3);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad4, 0x4);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad5, 0x5);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad6, 0x6);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad7, 0x7);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad8, 0x8);
        self.check_single_key(input, cpu, VirtualKeyCode::Numpad9, 0x9);



    }

}