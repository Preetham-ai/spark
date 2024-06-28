use std::io::BufReader;
use std::io::{stdin, Read};
use crossterm::terminal;
use tokio::task;

pub struct Editor {

}

pub fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

pub fn die(err: std::io::Error) {
    panic!("{}", err);
}

pub async fn meow() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = std::fs::File::open("src/audio/meow.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    
    sink.append(source);
    sink.sleep_until_end();
}

impl Editor {
    pub fn default() -> Self {            
        Editor{}            
    }
    #[tokio::main]
    pub async fn run(&self){
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
                        task::spawn(meow());
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