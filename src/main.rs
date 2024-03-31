use std::io::{self, stdin, stdout, Read};            
use crossterm::terminal;

fn to_ctrl_byte(c: char) -> u8 {
        let byte = c as u8;
        byte & 0b0001_1111
    }

// fn die(err: std::io::Error) {
//     panic!(err);
// }


fn main() {
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    for byte in stdin().bytes() {
        let byte = byte.unwrap();
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
    terminal::disable_raw_mode().expect("Could not turn off Raw mode");
}