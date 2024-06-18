use std::io::{self, stdin, stdout, Read};
use crossterm::terminal;

pub struct Editor {

}

pub fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

pub fn die(err: std::io::Error) {
    panic!("{}", err);
}

impl Editor {
    pub fn default() -> Self {            
        Editor{}            
    }
    pub fn run(&self){
        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
        for byte in stdin().bytes() {
            match byte {            
                Ok(byte) => {
                    let byte = byte;
                    let chars = byte as char;
    
                    if chars.is_control() {
                        println!("{:?} \r", byte);
                    } else {
                        println!("{:?} ({})\r", byte, chars);
                    }
                    if byte == to_ctrl_byte('q') {
                        break;
                    }
                }
                Err(err) => die(err), // Added match arm to handle Err case
            }
        }
        terminal::disable_raw_mode().expect("Could not turn off Raw mode");
    }
}