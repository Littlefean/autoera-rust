use crate::tools::cmd::input;

use super::{user::User, Spirit};

/**
 * 用户对战一个野生精灵
 */
pub struct BattleUserVsOneSpirit {
    user: User,
    spirit: Spirit,
}

impl BattleUserVsOneSpirit {
    pub fn new(user: User, spirit: Spirit) -> BattleUserVsOneSpirit {
        BattleUserVsOneSpirit {
            user: user,
            spirit: spirit,
        }
    }

    /// 用户和野生精灵对战一下
    ///
    /// 1. 我方选择招数
    /// 2. 对方选择招数
    /// 3. 程序判断谁先
    /// 4. 先行者出招，更新数据
    ///
    /// 5. 判断输赢
    /// 6. 后行者出招，更新数据
    /// 7. 判断输赢
    ///
    ////
    pub fn battle(&mut self) {
        // 假设我们是后手
        loop {
            if let Some(first_spirit) = self.user.get_first_spirit() {
                // 1 我方选择招数
                let user_select = input("请选择招数:\n[1] 选择招数 [2] 道具 [3] 逃跑".to_owned());
                match user_select.trim() {
                    "1" => {
                        let user_select = input("请选择招数".to_owned());
                    }
                    "2" => {
                        println!("[1] 补血道具");
                        println!("[2] 补技能数量道具");
                        println!("[3] 捕捉道具");
                        println!("[4] 其他特殊道具");
                        let user_select = input("请选择道具种类".to_owned());
                    }
                    "3" => {
                        return;
                    }
                    _ => (),
                }
                self.spirit.attack(first_spirit);

                if first_spirit.hp_current <= 0 {
                    println!("你失败了");
                    break;
                }

                first_spirit.attack(&mut self.spirit);

                if self.spirit.hp_current <= 0 {
                    print!("你胜利了");
                    break;
                }
            } else {
                // 处理背包中没有精灵的情况
                panic!("出现错误，背包中没有第一个宠物");
            }
        }
    }
}

// /**
//  * 用户同时对战两个及以上个数的野生精灵
//  */
// pub struct BattleUserVsSpirits {}

// /**
//  * 用户和其他用户对战，这里的其他用户指的是NPC
//  * NPC可能拥有多个精灵。但是会一个一个的上场。
//  * 不会同时上场
//  */
// pub struct BattleUserVsUser {}
