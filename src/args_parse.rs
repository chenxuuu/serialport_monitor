use clap::{AppSettings,Parser};

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
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub dtr: u8,
}
