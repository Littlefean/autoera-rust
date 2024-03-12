use std::io;
use std::io::Write;

use colored::*;

use crate::models::{self, Spirit};

/// 选主宠的过程
pub fn progress_choice_primary_spirit() -> Spirit {
    loop {
        println!("请选择您的主宠：");
        println!(
            "{} {}",
            "[1] 水系主宠：艾特慕斯".bright_blue(),
            models::SpeciesType::Hydro.emoji()
        );
        println!(
            "{} {}",
            "[2] 火系主宠：放克申".bright_yellow(),
            models::SpeciesType::Flare.emoji()
        );
        println!(
            "{} {}",
            "[3] 草系主宠：毛豆".bright_green(),
            models::SpeciesType::Floral.emoji()
        );

        let mut input = String::new();

        print!("请输入选项: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("无法读取输入");

        match input.trim() {
            "1" => {
                return Spirit::get_primary_hydro_spirit();
            }
            "2" => {
                return Spirit::get_primary_hydro_spirit();
            }
            "3" => {
                return Spirit::get_primary_hydro_spirit();
            }
            _ => {
                println!("请按照提示输入有效的选项！");
            }
        }
    };
}
