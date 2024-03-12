use std::io;
use std::io::Write;

pub fn input(hint: String) -> String {
    print!("{}", hint);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    return input;
}
