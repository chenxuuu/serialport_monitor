use std::{sync::mpsc::Sender, io::Read, thread};
use serialport::SerialPort;
use crate::args_parse::Args;

//读取串口数据的线程
pub fn read_loop(tx: Sender<Vec<u8>>, args : &Args) -> Box<dyn SerialPort> {
    let mut port = serialport::new(&args.port, args.baud_rate)
        .timeout(std::time::Duration::from_millis(10)).flow_control(serialport::FlowControl::None)
        .open().unwrap();
    port.write_request_to_send(args.rts != 0).unwrap();
    port.write_data_terminal_ready(args.dtr != 0).unwrap();
    let mut buff : [u8;4096] = [0;4096];
    let mut port_loop = port.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            let len = match port_loop.read(&mut buff) {
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => 0,
                Err(e) => {
                    println!("error: {:?}", e);
                    tx.send(vec![].to_vec()).unwrap();//发送空数据，表示串口异常
                    0
                },
                Ok(e) => e,
            };
            //没数据，不往下跑
            if len == 0 {
                continue;
            }
            tx.send(buff[0..len].to_vec()).unwrap();
        }
    });
    port
}
