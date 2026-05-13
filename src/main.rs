use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use sdl2;
use rand::Rng;
pub struct Chip8 {
    memory: [u8; 4096],
    st:u8, //sound timer
    dt:u8, //delay timer
    keypad:u16, 
    v:[u8;16], //Register for general purpose
    i:u16, //16 bit register used for registering a memmory address
    pc:u16, 
    stack:[u16; 0x10],
    sp:u8,
    framebuffer: [u8; (64*32)/8],
    tone: bool,
    time: isize,
    rng:StdRng,

}


impl Chip8 {
    pub fn new() -> Self {
        Self { memory: [0;4096], st: 0, dt: 0, keypad: 0, v: [0;16], i: 0, pc: 0x200, stack: [0;0x10], sp: 0, framebuffer: [0; 256], tone: false, time: 0, rng: StdRng::seed_from_u64(12345) }
    }

    


    pub fn op_00ee(&mut self) -> usize{
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];


        self.pc += 2;

        45

    }

    pub fn op_1nnn(&mut self, addr:u16) -> usize {

        self.pc = addr;
        45
    }

    pub fn op_2nnn(&mut self, addr: u16) -> usize {
        self.stack[self.sp as usize] = self.pc;
        self.sp +=1; 

        self.pc +=addr;

        45

    }

    pub fn op_3xkk(&mut self, x:usize,kk:u8) -> usize {
        if self.v[x] == kk {
             self.pc += 4;
             return 45
            }else{
                self.pc += 2;
                return 45
            }
        }
        
        
        pub fn op_4xkk(&mut self, x:usize, kk:u8) -> usize{
            if self.v[x] != kk {
                self.pc +=4;
            }else{
            self.pc += 2;
        }
        45
    }
    
    pub fn op_5xy0(&mut self, x:usize, y:u8) -> usize {
        if self.v[x] == self.v[y as usize] {
            self.pc += 4;
        }else{
            self.pc += 2;
        }
        45
    }
    pub fn op_6xkk(&mut self, x:usize, kk:u8) -> usize{
        self.v[x] = kk;
        self.pc += 2;
        return 45;
    }
    pub fn op_7xkk(&mut self,x:usize, y:u8) -> usize{
        self.v[x] = self.v[x].wrapping_add(y);

        self.pc +=2;

        return 45;
    }

    pub fn op_8xy0(&mut self, x:usize, y:u8) -> usize {
        
        self.v[x] = self.v[y as usize];
        self.pc += 2;
        45
    }

    pub fn op_8xy1(&mut self, x:usize, y:u8) -> usize {

        self.v[x] = self.v[x] | self.v[y as usize];
        self.pc +=2;  
        45
    }


    pub fn op_8xy2(&mut self, x:usize, y:u8) -> usize {
        
        self.v[x] = self.v[x] & self.v[y as usize];
        self.pc +=2;
        return 45;
    }

    pub fn op_8xy3(&mut self, x:usize, y:u8) -> usize {
        self.v[x] = self.v[x] ^ self.v[y as usize];
        self.pc +=2;
        return 45;
    }

    pub fn op_8xy4(&mut self, x:usize, y:u8) -> usize {
        let (result,overflow) = self.v[x].overflowing_add(self.v[y as usize]);
        self.v[0xF] = if overflow {1} else {0};
        self.v[x] = result;

        self.pc += 2;
        return 0;
    }

    pub fn op_8xy5(&mut self, x:usize, y:u8) -> usize {
        
        let (res, borrow) = self.v[x].overflowing_sub(self.v[y as usize]);
        self.v[0xF] = if !borrow {1} else {0};
        self.v[x] = res;
        self.pc += 2;
        return 45;
    }

    pub fn op_8xy6(&mut self, x:usize, y:u8) -> usize {
        let lsb = self.v[x] & 1;
        self.v[0xF] = if lsb == 1 {1} else {0};
        self.pc +=2;
        self.v[x] = self.v[x] * 2;
        return 45;
    }

    pub fn op_8xy7(&mut self, x:usize, y:u8) -> usize {
        let (res, borrow) = self.v[y as usize].overflowing_sub(self.v[x]);

        self.v[0xF] = if !borrow {1} else {0};
        self.v[x] = res;
        return 45;
    }

    pub fn op_8xye(&mut self, x:usize, y:u8) -> usize {
        let msb = self.v[x] & 7;
        self.v[0xF] = if msb == 1 {1} else {0};
        self.v[x] = self.v[x] * 2;
        self.pc += 2;
        return 0;
    }

    pub fn op_9xy0(&mut self, x:usize, y:u8) -> usize {
        if self.v[x] != self.v[y as usize]{
            self.pc +=2;
        }
        self.pc +=2;
        return 0;
    }

    pub fn op_annn(& mut self, addr:u16) -> usize{
        self.i = addr;
        self.pc +=2;
        45
    }

    pub fn op_bnnn(& mut self, addr:u16) -> usize {
        self.pc = addr + self.v[0] as u16;
        45
    }




    pub fn op_cxkk(& mut self,x:usize, kk: u8) -> usize{
        let rn:u8 = self.rng.random();
        self.v[x] = rn & kk;

        self.pc += 2;

        45
    }

    pub fn opex9e(& mut self, x:usize) -> usize {

        let key_to_check = self.v[x] & 0x0F;

        let is_pressed = (self.keypad >> key_to_check) & 1 == 1;

        self.pc += if is_pressed {4} else {2};

        return 45;
    }

    pub fn op_exa1(& mut self, x:usize) -> usize {
        let key_to_check = self.v[x] & 0x0F;
        let not_pressed = (self.keypad >> key_to_check) & 1 == 0;

        self.pc += if not_pressed {4} else {2};

        45
    }

    pub fn op_fx07(& mut self, x:usize ) -> usize {
        self.v[x] = self.dt;
        self.pc +=2;
        45
    }

    pub fn op_fx15(& mut self, x:usize) -> usize {
        self.dt = self.v[x];
        self.pc += 2;
        45
    }

    pub fn op_fx18(& mut self, x:usize) -> usize {
        self.st =self.v[x];
        self.pc += 2;
        45
    }

    pub fn op_fx1e(&mut self, x:usize) -> usize {
        self.i += self.v[x] as u16;
        self.pc += 2;
        45
    }

    pub fn op_fx55(&mut self, x:usize) -> usize {
        for i in 0..=x {
            self.memory[(self.i as usize)  +i] = self.v[i];
        }

        self.pc +=2;
        45
    }

    pub fn op_fx65(&mut self, x:usize) -> usize {
        for i in 0..x {
            self.v[i] = self.memory[(self.i as usize) + i];
        }

        self.pc += 2;
        45
    }

    pub fn op_fx33(&mut self, x:usize) -> usize {
        let full_value = self.v[x];
        let binary_hundreds = full_value/100;
        let binary_tens = (full_value/10) % 10;
        let binary_ones = full_value % 10;

        self.memory[self.i as usize] = binary_hundreds;
        self.memory[(self.i as usize) + 1] = binary_tens;
        self.memory[(self.i as usize) +2] = binary_ones;

        self.pc += 2;



        45
    }


    pub fn op_fx0a(&mut self, x:usize) -> usize{

        if let Some(key) = self.get_pressed_key() {
            self.v[x] = key;
            self.pc += 2;
        }else{

        }
        45
    }



    pub fn op_dyxn(&mut self, x:usize, y:usize, n:usize) -> usize {

        self.v[0xF] = 0;

        let pos_x: usize = self.v[x] as usize % 64;
        let pos_y: usize = self.v[y] as usize & 32;

        for row in 0..n {
            let sprite_byte: u8 = self.memory[self.i as usize + row];
            let target_y: usize = (pos_y + row) % 32;


            for col in 0..8 {

            let target_x: usize = (pos_x + col) % 64;
            let sprite_bit: bool = (sprite_byte >> (7 - col)) & 1 == 1;
            
           
            let pixel_index: usize = (target_y * 8) + (target_x / 8);
            let bit_offset: usize = 7 - (target_x % 8);
            let screen_bit: bool = (self.framebuffer[pixel_index] >> bit_offset) & 1 == 1;

            if sprite_bit {
                if screen_bit {
                    self.v[0xF] = 1;
                }
                self.framebuffer[pixel_index] ^= 1 << bit_offset;
            }
            }
        }
        self.pc+= 2;
        45
    }

    fn get_pressed_key(&self) -> Option<u8> {
         
         for i in 0..16 {
            if (self.keypad >> 8) & 1 == 1{
                return Some(i as u8);
            }
         }
         None
    }

    

}





fn open_window(sdl_context:sdl2::Sdl){
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let window: sdl2::video::Window = video_subsystem.window("rust-window", 800, 800).position_centered().build().unwrap();

    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running:loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _=>{}
            }
        }
    } 
}

fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    open_window(sdl_context)
    
}