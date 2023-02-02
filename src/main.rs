use std::io::{self, Write};

use clap::Parser;
mod args_parse;
use args_parse::Args;

fn main() {
    let args = Args::parse();
    let mut port = serialport::new(args.port, args.baud_rate)
        .timeout(std::time::Duration::from_millis(100)).flow_control(serialport::FlowControl::None)
        .open().unwrap();
    port.write_request_to_send(args.rts != 0).unwrap();
    port.write_data_terminal_ready(args.dtr != 0).unwrap();
    //清屏
    print!("\x1bc");
    io::stdout().flush().unwrap();
    loop {
        let mut buff : [u8;4096] = [0;4096];
        let len = match port.read(&mut buff) {
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
            e => e.unwrap(),
        };
        //没数据，不往下跑
        if len == 0 {
            continue;
        }
        let s = String::from_utf8_lossy(&buff[0..len]).into_owned();
        print!("{}",s);
        io::stdout().flush().unwrap();
    }
}
