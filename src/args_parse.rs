use clap::{AppSettings, Parser};

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Args {
    ///port name, example COM1,/dev/ttyUSB0
    #[clap(short, long, value_parser,value_name="port")]
    pub port: String,
    ///baud_rate; default 115200
    #[clap(short, long, value_parser,default_value_t= 115200)]
    pub baud_rate: u32,
    ///RTS status; 0 disable, 1 enable
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub rts: u8,
    ///DTR status; 0 disable, 1 enable
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub dtr: u8,
}

//获取参数
pub fn get_args() -> Args {
    match Args::try_parse(){
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
                dtr: 0,
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
            println!("please set dtr status (default 0, enable):");
            std::io::stdin().read_line(&mut buff).expect("read_line error!");
            if buff.trim().len() != 0 {
                a.dtr = buff.trim().parse().unwrap();
            }
            a
        }
    }
}
