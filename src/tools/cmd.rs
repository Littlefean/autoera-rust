use std::io;
use std::io::Write;

pub fn input(hint: String) -> String {
    print!("{}", hint);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    return String::from(input.trim());
}

pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

// /// 阻塞程序，让用户在控制台上做选择
// /// 返回选择的编号
// pub fn select_choice(select: Vec<String>) -> usize {

// }
