use std::io::{self, Write};

use clap::Parser;
mod args_parse;
use args_parse::Args;

fn main() {
    let args = match Args::try_parse(){
        Ok(a) => a,
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayHelp => Args::parse() ,
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayVersion => Args::parse() ,
        _ => {
            //没给参数，手动获取下
            let mut buff = String::new();
            let mut a = Args{
                port: String::new(),
                baud_rate: 115200,
                rts: 0,
                dtr: 1,
            };
            println!("your serial ports list:");
            let port_list = serialport::available_ports().unwrap();
            port_list.iter().for_each(|p| println!("{}",p.port_name));
            println!("please select your serial port (default first port if exist):");
            std::io::stdin().read_line(&mut buff).expect("read_line error!");
            a.port = if buff.trim().len() == 0 {
                if port_list.len() > 0{
                    port_list[0].port_name.clone()
                }
                else{
                    panic!("no port found!")
                }
            } else {
                buff.trim().to_string()
            };
            buff.clear();

            println!("please set a baud rate (default 115200):");
            std::io::stdin().read_line(&mut buff).expect("read_line error!");
            if buff.trim().len() != 0 {
                a.baud_rate = buff.trim().parse().unwrap();
            }
            buff.clear();
            println!("please set rts status (default 0, disable):");
            std::io::stdin().read_line(&mut buff).expect("read_line error!");
            if buff.trim().len() != 0 {
                a.rts = buff.trim().parse().unwrap();
            }
            buff.clear();
            println!("please set dtr status (default 1, enable):");
            std::io::stdin().read_line(&mut buff).expect("read_line error!");
            if buff.trim().len() != 0 {
                a.dtr = buff.trim().parse().unwrap();
            }
            a
        }
    };
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
