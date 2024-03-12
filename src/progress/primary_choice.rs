use colored::*;

use crate::models::Spirit;
use crate::tools::cmd::select_choice;

/// 选主宠的过程
pub fn progress_choice_primary_spirit() -> Spirit {
    loop {
        println!("请选择您的主宠：");
        let select = select_choice(vec![
            "水系主宠：艾特慕斯",
            "火系主宠：放克申",
            "草系主宠：毛豆",
        ]);

        match select {
            0 => {
                return Spirit::get_primary_hydro_spirit();
            }
            1 => {
                return Spirit::get_primary_flare_spirit();
            }
            2 => {
                return Spirit::get_primary_floral_spirit();
            }
            _ => {
                println!("请按照提示输入有效的选项！");
            }
        }
    }
}
