use std::{io::{self, Write}, sync::mpsc};

use crate::{uart_readout::read_loop, utf8_buffer::Utf8Buffer};
mod args_parse;
mod uart_readout;
mod key_listener;
mod utf8_buffer;

fn main() {
    let args = args_parse::get_args();

    let (tx, rx) = mpsc::channel();

    //清屏
    print!("\x1bc");
    io::stdout().flush().unwrap();

    let port = read_loop(tx.clone(), &args);

    key_listener::listen_keys(port);

    let mut last_buffer = vec![0u8; 20];

    loop {
        let recv = rx.recv().unwrap();
        if recv.len() == 0 {//串口异常
            println!("serial port error! exit!");
            break;
        }
        let recv = ([last_buffer.clone(),recv]).concat();
        last_buffer.clear();

        //看看末尾有没有不完整的utf8字符
        let mut u8b = Utf8Buffer::new();
        u8b.push_bytes(recv.clone());
        let income_count = u8b.incomplete_bytes_len();
        let s = if income_count > 0 {
            last_buffer.extend_from_slice(&recv[recv.len() - income_count..]);
            String::from_utf8_lossy(&recv[..recv.len() - income_count]).to_string()
        } else {
            String::from_utf8_lossy(&recv).to_string()
        };

        io::stdout().write(s.as_bytes()).unwrap();
        io::stdout().flush().ok();
    }
}
