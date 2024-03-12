use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use std::io;
use std::io::Write;

pub fn input(hint: String) -> String {
    print!("{}", hint);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    return String::from(input.trim());
}

/// 清空控制台
pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

/// # 获得用户选择
///
/// 这个函数一执行，就会让用户做一个选择，会打印出若干行选项，左侧带有标号，
/// 用户通过按上下方向键来切换选项，按下之后，控制台上会实时刷新选项左侧的选择状态。
/// 然后按下回车确认选择。
///
pub fn select_choice(select: Vec<&str>) -> usize {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let mut selected_index = 0;

    loop {
        // 清空终端并重新打印选项列表
        clear_console();
        println!("按↑↓切换选项，Enter选择选项");
        print_options(&select, selected_index);

        if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < select.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break; // 用户按下回车确认选择，退出循环
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    selected_index
}

fn print_options(select: &Vec<&str>, selected_index: usize) {
    for (i, option) in select.iter().enumerate() {
        if i == selected_index {
            println!("* {} {}", i + 1, option); // 已选择的选项
        } else {
            println!("  {} {}", i + 1, option); // 未选择的选项
        }
    }
}
