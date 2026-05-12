use rand::rngs::StdRng;
use rand::SeedableRng;
use sdl2;

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
        return 0;
    }

    pub fn op_8xy6(&mut self, x:usize, y:u8) -> usize {
        return 0;
    }

    pub fn op_8xy7(&mut self, x:usize, y:u8) -> usize {
        return 0;
    }

    pub fn op_8xye(&mut self, x:usize, y:u8) -> usize {
        return 0;
    }

    pub fn op_9xy0(&mut self, x:usize, y:u8) -> usize {
        if self.v[x] != self.v[y as usize]{
            self.pc +=2;
        }
        self.pc +=2;
        return 0;
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