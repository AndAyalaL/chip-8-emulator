use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use sdl2;
use rand::Rng;




const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0x80, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
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

const FRAME_TIME: isize = 1000;
const MEM_SIZE: usize = 4096;

impl Chip8 {
    pub fn new() -> Self {
        let mut new_chip = Self { memory: [0;4096], st: 0, dt: 0, keypad: 0, v: [0;16], i: 0, pc: 0x200, stack: [0;0x10], sp: 0, framebuffer: [0; 256], tone: false, time: 0, rng: StdRng::seed_from_u64(12345) };
    
        new_chip.memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
        new_chip
    }

    

    pub fn op_00e0(&mut self) -> usize {
        self.framebuffer.fill(0);

        self.pc += 2;
        45
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

        self.pc =addr;

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
    
    pub fn op_5xy0(&mut self, x:usize, y:usize) -> usize {
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

    pub fn op_8xy0(&mut self, x:usize, y:usize) -> usize {
        
        self.v[x] = self.v[y as usize];
        self.pc += 2;
        45
    }

    pub fn op_8xy1(&mut self, x:usize, y:usize) -> usize {

        self.v[x] = self.v[x] | self.v[y as usize];
        self.pc +=2;  
        45
    }


    pub fn op_8xy2(&mut self, x:usize, y:usize) -> usize {
        
        self.v[x] = self.v[x] & self.v[y as usize];
        self.pc +=2;
        return 45;
    }

    pub fn op_8xy3(&mut self, x:usize, y:usize) -> usize {
        self.v[x] = self.v[x] ^ self.v[y as usize];
        self.pc +=2;
        return 45;
    }

    pub fn op_8xy4(&mut self, x:usize, y:usize) -> usize {
        let (result,overflow) = self.v[x].overflowing_add(self.v[y as usize]);
        self.v[0xF] = if overflow {1} else {0};
        self.v[x] = result;

        self.pc += 2;
        return 45;
    }

    pub fn op_8xy5(&mut self, x:usize, y:usize) -> usize {
        
        let (res, borrow) = self.v[x].overflowing_sub(self.v[y as usize]);
        self.v[0xF] = if !borrow {1} else {0};
        self.v[x] = res;
        self.pc += 2;
        return 45;
    }

    pub fn op_8xy6(&mut self, x:usize) -> usize {
        let lsb = self.v[x] & 1;
        self.v[0xF] = if lsb == 1 {1} else {0};
        self.v[x]  >>= 2;
        self.pc +=2;
        return 45;
    }

    pub fn op_8xy7(&mut self, x:usize, y:usize) -> usize {
        let (res, borrow) = self.v[y as usize].overflowing_sub(self.v[x]);

        self.v[0xF] = if !borrow {1} else {0};
        self.v[x] = res;
        self.pc += 2;
        return 45;
    }

    pub fn op_8xye(&mut self, x:usize) -> usize {
        let msb = (self.v[x] >> 7) &1;
        self.v[0xF] = if msb == 1 {1} else {0};
        self.v[x] = self.v[x].wrapping_mul(2);
        self.pc += 2;
        return 45;
    }

    pub fn op_9xy0(&mut self, x:usize, y:usize) -> usize {
        if self.v[x] != self.v[y as usize]{
            self.pc +=2;
        }
        self.pc +=2;
        return 45;
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

    pub fn op_ex9e(& mut self, x:usize) -> usize {

        let key_to_check = self.v[x] & 0x0F;

        let is_pressed = (self.keypad >> key_to_check) & 1 == 1;

        if is_pressed {
            self.pc += 4;
        } else {
            self.pc += 2; 
        }

        return 45;
    }

    pub fn op_exa1(& mut self, x:usize) -> usize {
        let key_to_check = self.v[x] & 0x0F;
        let not_pressed = (self.keypad >> key_to_check) & 1 == 0;

        if not_pressed {
            self.pc += 4;
        } else {
            self.pc += 2; 
        }

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
        for i in 0..=x {
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

    
    
    pub fn op_dxyn(&mut self, x:usize, y:usize, n:usize) -> usize {
        
            self.v[0xF] = 0;

            let pos_x: usize = self.v[x] as usize % 64;
            let pos_y: usize = self.v[y] as usize % 32;

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

    pub fn op_fx29(&mut self, x:usize) -> usize{
        self.i = (self.v[x] as u16) * 5;

        self.pc += 2;

        return 45 as usize;
    }
    fn get_pressed_key(&self) -> Option<u8> {
         
         for i in 0..16 {
            if (self.keypad >> i) & 1 == 1{
                return Some(i as u8);
            }
         }
         None
    }



    pub fn execute(&mut self, opcode:u16) -> usize{
        let x = ((opcode & 0x0F00 ) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;


        match (opcode & 0xF000) >> 12 {
        0x0 => match opcode & 0x00FF {
            0xE0 => self.op_00e0(),
            0xEE => self.op_00ee(),      
            _ => panic!("Unknown 0xxx opcode: {:04X}", opcode),
        },
        0x1 => self.op_1nnn(nnn),        
        0x2 => self.op_2nnn(nnn),       
        0x3 => self.op_3xkk(x, kk),      
        0x4 => self.op_4xkk(x, kk),      
        0x5 => self.op_5xy0(x, y),       
        0x6 => self.op_6xkk(x, kk),      
        0x7 => self.op_7xkk(x, kk),      
        0x8 => match opcode & 0x000F {
            0x0 => self.op_8xy0(x, y),
            0x1 => self.op_8xy1(x, y),  
            0x2 => self.op_8xy2(x, y),  
            0x3 => self.op_8xy3(x, y), 
            0x4 => self.op_8xy4(x, y),   
            0x5 => self.op_8xy5(x, y),  
            0x6 => self.op_8xy6(x),  
            0x7 => self.op_8xy7(x, y),
            0xE => self.op_8xye(x),  
            _ => panic!("Unknown 8xxx opcode: {:04X}", opcode),
        },
        0x9 => self.op_9xy0(x, y), 
        0xA => self.op_annn(nnn),
        0xB => self.op_bnnn(nnn),
        0xC => self.op_cxkk(x, kk),
        0xD => self.op_dxyn(x, y, n),   
        0xE => match opcode & 0x00FF {
            0x9E => self.op_ex9e(x),   
            0xA1 => self.op_exa1(x), 
            _ => panic!("Unknown Exxx opcode: {:04X}", opcode),
        },
        0xF => match opcode & 0x00FF {
            0x07 => self.op_fx07(x),
            0x0A => self.op_fx0a(x),     
            0x15 => self.op_fx15(x),     
            0x18 => self.op_fx18(x),     
            0x1E => self.op_fx1e(x),     
            0x29 => self.op_fx29(x),     
            0x33 => self.op_fx33(x),
            0x55 => self.op_fx55(x),     
            0x65 => self.op_fx65(x),     
            _ => panic!("Unknown Fxxx opcode: {:04X}", opcode),
        },
        _ => panic!("Opcode not implemented: {:04X}", opcode),
    }
}



    pub fn frame(&mut self, keypay:u16) -> Result<(),String> {
        
        self.keypad = keypay;

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
            self.tone = true;
        }else{
            self.tone = false;
        }


        self.time += FRAME_TIME;



        while self.time > 0 {

            if self.pc as usize >= MEM_SIZE - 1 {
                return Err(format!("PC out of bounds: {:04X}", self.pc));
            }

            let b1 = self.memory[self.pc as usize] as u16;

            let b2 = self.memory[self.pc as usize + 1] as u16;

            let opcode = (b1 << 8) | b2;

            let cycles_used = self.execute(opcode);
            self.time -= cycles_used as isize;
        }

        return Ok(());
    }


    pub fn load_rom(&mut self, data: &[u8])  {
        let start = 0x200;
        let end = 0x200 + data.len();

       
        self.memory[start..end].copy_from_slice(data);
    }
    

}


fn map_keycode(keycode: sdl2::keyboard::Keycode) -> Option<u8> {
    match keycode {
        sdl2::keyboard::Keycode::Num1 => Some(0x1),
        sdl2::keyboard::Keycode::Num2 => Some(0x2),
        sdl2::keyboard::Keycode::Num3 => Some(0x3),
        sdl2::keyboard::Keycode::Num4 => Some(0xC),
        sdl2::keyboard::Keycode::Q => Some(0x4),
        sdl2::keyboard::Keycode::W => Some(0x5),
        sdl2::keyboard::Keycode::E => Some(0x6),
        sdl2::keyboard::Keycode::R => Some(0xD),
        sdl2::keyboard::Keycode::A => Some(0x7),
        sdl2::keyboard::Keycode::S => Some(0x8),
        sdl2::keyboard::Keycode::D => Some(0x9),
        sdl2::keyboard::Keycode::F => Some(0xE),
        sdl2::keyboard::Keycode::Z => Some(0xA),
        sdl2::keyboard::Keycode::X => Some(0x0),
        sdl2::keyboard::Keycode::C => Some(0xB),
        sdl2::keyboard::Keycode::V => Some(0xF),
        _ => None,
    }
}




fn main() {
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();

    let mut chip8 = Chip8::new();

    let rom_bytes = std::fs::read("assets/PONG.ch8").expect("Yo duuuuudeeeee, i couldn't load the ROM file");
    chip8.load_rom(&rom_bytes);
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
    let window: sdl2::video::Window = video_subsystem.window("rust-window", 1920, 1080).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut keypad_state: u16 = 0;

    'running:loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {keycode:Some(k),..} => {
                    if let Some(key) = map_keycode(k) {
                        keypad_state |= 1 << key;
                    }
                }
                sdl2::event::Event::KeyUp { keycode: Some(k), .. } => {
                    if let Some(key) = map_keycode(k) {
                        keypad_state &= !(1 << key);
                    }
                }
                _=>{}
            }
        }

        if let Err(e) = chip8.frame(keypad_state) {
            println!("Emulation Error: {}", e);
            break;
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,255));



        for y in 0..32 {
            for x in 0..64 {
                let pixel_index = (y * 8) + (x / 8);
                let bit_offset = 7 - (x % 8);
                let is_on = (chip8.framebuffer[pixel_index] >> bit_offset) & 1 == 1;

                if is_on {
                    canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 30, y as i32 * 10, 10, 10)).ok();
                }
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(60));
    } 
}