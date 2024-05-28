

pub const SCREEN_WIDTH:usize = 64;
pub const SCREEN_HEIGHT:usize = 32;

const RAM_SIZE:usize = 4096;
const NUM_REGS:usize = 16;
const STACK_SIZE:usize = 16;
const NUMS_KEYS:usize = 16;
const START_ADDR:u16 = 0x200;
const FONT_SIZE:usize = 80;

const FONTSET: [u8; FONT_SIZE] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];


pub struct Emu{
    pc: u16,
    ram:[u8;RAM_SIZE],
    screen:[bool;SCREEN_WIDTH * SCREEN_HEIGHT],
    v_regs:[u8;NUM_REGS], //used for faster op
    i_regs: u16, //used for indexing into ram for read n write
    sp:u16,
    stack:[u16;STACK_SIZE],
    keys:[bool;NUMS_KEYS],
    dt:u8,
    st:u8,

}

impl Emu{

    pub fn new()->Self {
        Self { 
            pc: START_ADDR,
            ram:[0;RAM_SIZE],
            screen:[false;SCREEN_WIDTH * SCREEN_HEIGHT],
            v_regs:[0;NUM_REGS], 
            i_regs: 0, 
            sp:0,
            stack:[0;STACK_SIZE],
            keys:[false;NUMS_KEYS],
            dt:0,
            st:0,
            }

            new_emu.ram[..FONT_SIZE].copy_from_slice(&FONTSET);
            new_emu  
        }
       
        fn push(&mut self,val:u16){
         self.stack[self.sp as usize] = val;
         self.sp+=1; //overflow error handling left
        }

        fn pop(&mut self)->u16{
            self.sp-=1;
            self.stack[self.sp as usize] //underflow error handing left
        }

    }


