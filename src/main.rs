use std::io::{self, Write, Read};
mod args_parse;

fn main() {
    let args = args_parse::get_args();
    let mut port = serialport::new(args.port, args.baud_rate)
        .timeout(std::time::Duration::from_millis(10)).flow_control(serialport::FlowControl::None)
        .open().unwrap();
    port.write_request_to_send(args.rts != 0).unwrap();
    port.write_data_terminal_ready(args.dtr != 0).unwrap();
    //清屏
    print!("\x1bc");
    io::stdout().flush().unwrap();
    let mut buff : [u8;4096] = [0;4096];
    let mut read_buff = [0;4096];
    loop {
        let len = match port.read(&mut buff) {
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
            e => e.unwrap(),
        };
        let read_len = std::io::stdin().read(&mut read_buff).unwrap();
        if read_len != 0 {
            println!("read:{}",String::from_utf8_lossy(&read_buff[0..read_len]));
            port.write(&read_buff[0..read_len]).unwrap();
        }
        //没数据，不往下跑
        if len == 0 {
            continue;
        }
        io::stdout().write(&buff[0..len]).unwrap();
        io::stdout().flush().unwrap();
    }
}
