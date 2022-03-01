
pub struct TimerModule {
    sound_timer: u8,
    delay_timer: u8,
    sound_flag: bool,
    delay_flag: bool
}
impl TimerModule {

    //initialize timers
    pub fn new() -> Self {

        Self {
            sound_timer: 0,
            delay_timer: 0,
            sound_flag: false,
            delay_flag: false
        }

    }

    //to be called at 60hz
    pub fn update(&mut self) {

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            self.sound_flag = true; //flag is up if sound_timer is active ( > 0 )
            println!("---ST: {}", self.sound_timer);
        } else {
            self.sound_flag = false;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            self.delay_flag = true; //flag is up if delay_timer is active ( > 0 )
            println!("---DT: {}", self.delay_timer);
        } else {
            self.delay_flag = false;
        }

    }

    pub fn get_sound_flag(&self) -> bool {
        self.sound_flag
    }


    pub fn get_sound_register(&self) -> u8 { self.sound_timer }
    pub fn get_delay_register(&self) -> u8 { self.delay_timer }


    pub fn set_sound_register(&mut self, new_val: u8) {
        self.sound_timer = new_val;
        println!("---ST: {}", self.sound_timer);
        if self.sound_timer > 0 {
            self.sound_flag = true;
        }
    }

    pub fn set_delay_register(&mut self, new_val: u8) {
        self.delay_timer = new_val;
        println!("---DT {}", self.delay_timer);
        if self.delay_timer > 0 {
            self.delay_flag = true;
        }
    }

}


