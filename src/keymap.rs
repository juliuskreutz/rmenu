// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_upper_case_globals)]
const XK_BackSpace: u32 = 0xFF08;
const XK_Return: u32 = 0xFF0D;
const XK_Escape: u32 = 0xFF1B;
const XK_Left: u32 = 0xFF51;
const XK_Right: u32 = 0xFF53;
const XK_space: u32 = 0x020;
const XK_0: u32 = 0x030;
const XK_1: u32 = 0x031;
const XK_2: u32 = 0x032;
const XK_3: u32 = 0x033;
const XK_4: u32 = 0x034;
const XK_5: u32 = 0x035;
const XK_6: u32 = 0x036;
const XK_7: u32 = 0x037;
const XK_8: u32 = 0x038;
const XK_9: u32 = 0x039;
const XK_A: u32 = 0x041;
const XK_B: u32 = 0x042;
const XK_C: u32 = 0x043;
const XK_D: u32 = 0x044;
const XK_E: u32 = 0x045;
const XK_F: u32 = 0x046;
const XK_G: u32 = 0x047;
const XK_H: u32 = 0x048;
const XK_I: u32 = 0x049;
const XK_J: u32 = 0x04a;
const XK_K: u32 = 0x04b;
const XK_L: u32 = 0x04c;
const XK_M: u32 = 0x04d;
const XK_N: u32 = 0x04e;
const XK_O: u32 = 0x04f;
const XK_P: u32 = 0x050;
const XK_Q: u32 = 0x051;
const XK_R: u32 = 0x052;
const XK_S: u32 = 0x053;
const XK_T: u32 = 0x054;
const XK_U: u32 = 0x055;
const XK_V: u32 = 0x056;
const XK_W: u32 = 0x057;
const XK_X: u32 = 0x058;
const XK_Y: u32 = 0x059;
const XK_Z: u32 = 0x05a;
const XK_a: u32 = 0x061;
const XK_b: u32 = 0x062;
const XK_c: u32 = 0x063;
const XK_d: u32 = 0x064;
const XK_e: u32 = 0x065;
const XK_f: u32 = 0x066;
const XK_g: u32 = 0x067;
const XK_h: u32 = 0x068;
const XK_i: u32 = 0x069;
const XK_j: u32 = 0x06a;
const XK_k: u32 = 0x06b;
const XK_l: u32 = 0x06c;
const XK_m: u32 = 0x06d;
const XK_n: u32 = 0x06e;
const XK_o: u32 = 0x06f;
const XK_p: u32 = 0x070;
const XK_q: u32 = 0x071;
const XK_r: u32 = 0x072;
const XK_s: u32 = 0x073;
const XK_t: u32 = 0x074;
const XK_u: u32 = 0x075;
const XK_v: u32 = 0x076;
const XK_w: u32 = 0x077;
const XK_x: u32 = 0x078;
const XK_y: u32 = 0x079;
const XK_z: u32 = 0x07a;

pub struct Keymap {
    keysyms: Vec<u32>,
    keysyms_per_keycode: u8,
    min_keycode: u8,
}

impl Keymap {
    pub fn new(keysyms: Vec<u32>, keysyms_per_keycode: u8, min_keycode: u8) -> Self {
        Self {
            keysyms,
            keysyms_per_keycode,
            min_keycode,
        }
    }

    pub fn get_keysym(&self, keycode: u8, state: u16) -> u32 {
        self.keysyms[state as usize
            + (keycode - self.min_keycode) as usize * self.keysyms_per_keycode as usize]
    }

    pub const fn get_key(&self, keysym: u32) -> &'static str {
        match keysym {
            XK_space => " ",
            XK_0 => "0",
            XK_1 => "1",
            XK_2 => "2",
            XK_3 => "3",
            XK_4 => "4",
            XK_5 => "5",
            XK_6 => "6",
            XK_7 => "7",
            XK_8 => "8",
            XK_9 => "9",
            XK_A => "A",
            XK_B => "B",
            XK_C => "C",
            XK_D => "D",
            XK_E => "E",
            XK_F => "F",
            XK_G => "G",
            XK_H => "H",
            XK_I => "I",
            XK_J => "J",
            XK_K => "K",
            XK_L => "L",
            XK_M => "M",
            XK_N => "N",
            XK_O => "O",
            XK_P => "P",
            XK_Q => "Q",
            XK_R => "R",
            XK_S => "S",
            XK_T => "T",
            XK_U => "U",
            XK_V => "V",
            XK_W => "W",
            XK_X => "X",
            XK_Y => "Y",
            XK_Z => "Z",
            XK_a => "a",
            XK_b => "b",
            XK_c => "c",
            XK_d => "d",
            XK_e => "e",
            XK_f => "f",
            XK_g => "g",
            XK_h => "h",
            XK_i => "i",
            XK_j => "j",
            XK_k => "k",
            XK_l => "l",
            XK_m => "m",
            XK_n => "n",
            XK_o => "o",
            XK_p => "p",
            XK_q => "q",
            XK_r => "r",
            XK_s => "s",
            XK_t => "t",
            XK_u => "u",
            XK_v => "v",
            XK_w => "w",
            XK_x => "x",
            XK_y => "y",
            XK_z => "z",
            _ => "",
        }
    }
}

pub fn is_escape(keysym: u32) -> bool {
    XK_Escape == keysym
}

pub fn is_return(keysym: u32) -> bool {
    XK_Return == keysym
}

pub fn is_backspace(keysym: u32) -> bool {
    XK_BackSpace == keysym
}

pub fn is_left(keysym: u32) -> bool {
    XK_Left == keysym
}

pub fn is_right(keysym: u32) -> bool {
    XK_Right == keysym
}
