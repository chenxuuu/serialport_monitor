use std::{io::{self, Write}, sync::mpsc};

use crate::uart_readout::read_loop;
mod args_parse;
mod uart_readout;
mod key_listener;

fn main() {
    let args = args_parse::get_args();

    let (tx, rx) = mpsc::channel();

    //清屏
    print!("\x1bc");
    io::stdout().flush().unwrap();

    let port = read_loop(tx.clone(), &args);

    key_listener::listen_keys(port);

    loop {
        let r = rx.recv().unwrap();
        if r.len() == 0 {//串口异常
            println!("serial port error! exit!");
            break;
        }
        let s = String::from_utf8_lossy(&r).to_string();
        io::stdout().write(s.as_bytes()).unwrap();
        io::stdout().flush().ok();
    }
}
