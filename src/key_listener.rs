use std::thread;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use serialport::SerialPort;

pub fn listen_keys(mut port: Box<dyn SerialPort>) {
    //ctrl+c单独处理
    let mut ctrlc_port = port.try_clone().unwrap();
    ctrlc::set_handler(move || {
        ctrlc_port.write(&get_key_data(KeyCode::Char('c'), true)).unwrap();
    })
    .expect("Error setting Ctrl-C handler");
    //port.write(&get_special_key_codes("[?1;0c")).unwrap();

    thread::spawn(move || {
        loop {
            let mut code = vec![];
            // `read()` blocks until an `Event` is available
            match crossterm::event::read().unwrap() {
                //按键事件
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        code = get_key_data(event.code, event.modifiers == KeyModifiers::CONTROL);
                    }
                }
                // Event::Resize(columns, _) => {
                //     code = get_special_key_codes(
                //         if columns >= 132 {
                //             "[?3h"
                //         } else {
                //             "[?3l"
                //         }
                //     );
                // },
                _ => (),
            };
            if code.len() > 0 {
                port.write(&code).unwrap();
            }
        }
    });
}

//返回按键数据
fn get_key_data(key: KeyCode, ctrl: bool) -> Vec<u8> {
    match key {
        //数据来源：https://redirect.cs.umbc.edu/portal/help/theory/ascii.txt
        //ctrl组合键
        KeyCode::Char('@') if ctrl => vec![0x00],
        KeyCode::Char(' ') if ctrl => vec![0x00],
        KeyCode::Char('A') | KeyCode::Char('a') if ctrl => vec![0x01],
        KeyCode::Char('B') | KeyCode::Char('b') if ctrl => vec![0x02],
        KeyCode::Char('C') | KeyCode::Char('c') if ctrl => vec![0x03],
        KeyCode::Char('D') | KeyCode::Char('d') if ctrl => vec![0x04],
        KeyCode::Char('E') | KeyCode::Char('e') if ctrl => vec![0x05],
        KeyCode::Char('F') | KeyCode::Char('f') if ctrl => vec![0x06],
        KeyCode::Char('G') | KeyCode::Char('g') if ctrl => vec![0x07],
        KeyCode::Char('H') | KeyCode::Char('h') if ctrl => vec![0x08],
        KeyCode::Char('I') | KeyCode::Char('i') if ctrl => vec![0x09],
        KeyCode::Char('J') | KeyCode::Char('j') if ctrl => vec![0x0A],
        KeyCode::Char('K') | KeyCode::Char('k') if ctrl => vec![0x0B],
        KeyCode::Char('L') | KeyCode::Char('l') if ctrl => vec![0x0C],
        KeyCode::Char('M') | KeyCode::Char('m') if ctrl => vec![0x0D],
        KeyCode::Char('N') | KeyCode::Char('n') if ctrl => vec![0x0E],
        KeyCode::Char('O') | KeyCode::Char('o') if ctrl => vec![0x0F],
        KeyCode::Char('P') | KeyCode::Char('p') if ctrl => vec![0x10],
        KeyCode::Char('Q') | KeyCode::Char('q') if ctrl => vec![0x11],
        KeyCode::Char('R') | KeyCode::Char('r') if ctrl => vec![0x12],
        KeyCode::Char('S') | KeyCode::Char('s') if ctrl => vec![0x13],
        KeyCode::Char('T') | KeyCode::Char('t') if ctrl => vec![0x14],
        KeyCode::Char('U') | KeyCode::Char('u') if ctrl => vec![0x15],
        KeyCode::Char('V') | KeyCode::Char('v') if ctrl => vec![0x16],
        KeyCode::Char('W') | KeyCode::Char('w') if ctrl => vec![0x17],
        KeyCode::Char('X') | KeyCode::Char('x') if ctrl => vec![0x18],
        KeyCode::Char('Y') | KeyCode::Char('y') if ctrl => vec![0x19],
        KeyCode::Char('Z') | KeyCode::Char('z') if ctrl => vec![0x1A],
        KeyCode::Char('[') if ctrl => vec![0x1B],
        KeyCode::Char('\\') if ctrl => vec![0x1C],
        KeyCode::Char(']') if ctrl => vec![0x1D],
        KeyCode::Char('^') if ctrl => vec![0x1E],
        KeyCode::Char('_') if ctrl => vec![0x1F],
        //普通字符按键
        KeyCode::Char(' ') => vec![0x20],
        KeyCode::Char('!') => vec![0x21],
        KeyCode::Char('"') => vec![0x22],
        KeyCode::Char('#') => vec![0x23],
        KeyCode::Char('$') => vec![0x24],
        KeyCode::Char('%') => vec![0x25],
        KeyCode::Char('&') => vec![0x26],
        KeyCode::Char('\'') => vec![0x27],
        KeyCode::Char('(') => vec![0x28],
        KeyCode::Char(')') => vec![0x29],
        KeyCode::Char('*') => vec![0x2A],
        KeyCode::Char('+') => vec![0x2B],
        KeyCode::Char(',') => vec![0x2C],
        KeyCode::Char('-') => vec![0x2D],
        KeyCode::Char('.') => vec![0x2E],
        KeyCode::Char('/') => vec![0x2F],
        KeyCode::Char('0') => vec![0x30],
        KeyCode::Char('1') => vec![0x31],
        KeyCode::Char('2') => vec![0x32],
        KeyCode::Char('3') => vec![0x33],
        KeyCode::Char('4') => vec![0x34],
        KeyCode::Char('5') => vec![0x35],
        KeyCode::Char('6') => vec![0x36],
        KeyCode::Char('7') => vec![0x37],
        KeyCode::Char('8') => vec![0x38],
        KeyCode::Char('9') => vec![0x39],
        KeyCode::Char(':') => vec![0x3A],
        KeyCode::Char(';') => vec![0x3B],
        KeyCode::Char('<') => vec![0x3C],
        KeyCode::Char('=') => vec![0x3D],
        KeyCode::Char('>') => vec![0x3E],
        KeyCode::Char('?') => vec![0x3F],
        KeyCode::Char('@') => vec![0x40],
        KeyCode::Char('A') => vec![0x41],
        KeyCode::Char('B') => vec![0x42],
        KeyCode::Char('C') => vec![0x43],
        KeyCode::Char('D') => vec![0x44],
        KeyCode::Char('E') => vec![0x45],
        KeyCode::Char('F') => vec![0x46],
        KeyCode::Char('G') => vec![0x47],
        KeyCode::Char('H') => vec![0x48],
        KeyCode::Char('I') => vec![0x49],
        KeyCode::Char('J') => vec![0x4A],
        KeyCode::Char('K') => vec![0x4B],
        KeyCode::Char('L') => vec![0x4C],
        KeyCode::Char('M') => vec![0x4D],
        KeyCode::Char('N') => vec![0x4E],
        KeyCode::Char('O') => vec![0x4F],
        KeyCode::Char('P') => vec![0x50],
        KeyCode::Char('Q') => vec![0x51],
        KeyCode::Char('R') => vec![0x52],
        KeyCode::Char('S') => vec![0x53],
        KeyCode::Char('T') => vec![0x54],
        KeyCode::Char('U') => vec![0x55],
        KeyCode::Char('V') => vec![0x56],
        KeyCode::Char('W') => vec![0x57],
        KeyCode::Char('X') => vec![0x58],
        KeyCode::Char('Y') => vec![0x59],
        KeyCode::Char('Z') => vec![0x5A],
        KeyCode::Char('[') => vec![0x5B],
        KeyCode::Char('\\') => vec![0x5C],
        KeyCode::Char(']') => vec![0x5D],
        KeyCode::Char('^') => vec![0x5E],
        KeyCode::Char('_') => vec![0x5F],
        KeyCode::Char('`') => vec![0x60],
        KeyCode::Char('a') => vec![0x61],
        KeyCode::Char('b') => vec![0x62],
        KeyCode::Char('c') => vec![0x63],
        KeyCode::Char('d') => vec![0x64],
        KeyCode::Char('e') => vec![0x65],
        KeyCode::Char('f') => vec![0x66],
        KeyCode::Char('g') => vec![0x67],
        KeyCode::Char('h') => vec![0x68],
        KeyCode::Char('i') => vec![0x69],
        KeyCode::Char('j') => vec![0x6A],
        KeyCode::Char('k') => vec![0x6B],
        KeyCode::Char('l') => vec![0x6C],
        KeyCode::Char('m') => vec![0x6D],
        KeyCode::Char('n') => vec![0x6E],
        KeyCode::Char('o') => vec![0x6F],
        KeyCode::Char('p') => vec![0x70],
        KeyCode::Char('q') => vec![0x71],
        KeyCode::Char('r') => vec![0x72],
        KeyCode::Char('s') => vec![0x73],
        KeyCode::Char('t') => vec![0x74],
        KeyCode::Char('u') => vec![0x75],
        KeyCode::Char('v') => vec![0x76],
        KeyCode::Char('w') => vec![0x77],
        KeyCode::Char('x') => vec![0x78],
        KeyCode::Char('y') => vec![0x79],
        KeyCode::Char('z') => vec![0x7A],
        KeyCode::Char('{') => vec![0x7B],
        KeyCode::Char('|') => vec![0x7C],
        KeyCode::Char('}') => vec![0x7D],
        KeyCode::Char('~') => vec![0x7E],
        //中文啥的
        KeyCode::Char(any) => any.to_string().as_bytes().to_vec(),
        
        //特殊按键
        KeyCode::Delete => vec![0x7F],
        KeyCode::Backspace => vec![0x08],
        KeyCode::Tab => vec![0x09],
        KeyCode::Enter => vec![0x0D],
        KeyCode::Pause=> vec![0x1A],
        KeyCode::Esc => vec![0x1B],
        
        // https://learn.microsoft.com/zh-cn/windows/console/console-virtual-terminal-sequences
        KeyCode::Up    if ctrl => get_special_key_codes("[1;5A"),
        KeyCode::Down  if ctrl => get_special_key_codes("[1;5B"),
        KeyCode::Right if ctrl => get_special_key_codes("[1;5C"),
        KeyCode::Left  if ctrl => get_special_key_codes("[1;5D"),
        KeyCode::Up    => get_special_key_codes("[A"),
        KeyCode::Down  => get_special_key_codes("[B"),
        KeyCode::Right => get_special_key_codes("[C"),
        KeyCode::Left  => get_special_key_codes("[D"),
        KeyCode::Home => get_special_key_codes("[H"),
        KeyCode::End  => get_special_key_codes("[F"),
        KeyCode::Insert => get_special_key_codes("[2~"),
        KeyCode::PageUp => get_special_key_codes("[5~"),
        KeyCode::PageDown => get_special_key_codes("[6~"),
        KeyCode::F(1) => get_special_key_codes("OP"),
        KeyCode::F(2) => get_special_key_codes("OQ"),
        KeyCode::F(3) => get_special_key_codes("OR"),
        KeyCode::F(4) => get_special_key_codes("OS"),
        KeyCode::F(5) => get_special_key_codes("[15~"),
        KeyCode::F(6) => get_special_key_codes("[17~"),
        KeyCode::F(7) => get_special_key_codes("[18~"),
        KeyCode::F(8) => get_special_key_codes("[19~"),
        KeyCode::F(9) => get_special_key_codes("[20~"),
        KeyCode::F(10) => get_special_key_codes("[21~"),
        KeyCode::F(11) => get_special_key_codes("[23~"),
        KeyCode::F(12) => get_special_key_codes("[24~"),


        _ => vec![], //其他按键不处理
    }
}

fn get_special_key_codes(s : &str) -> Vec<u8> {
    let mut v = vec![0x1B];
    let mut b = s.as_bytes().to_vec();
    v.append(&mut b);
    v
}
