use crate::tools::cmd::select_choice;

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
            user,
            spirit,
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
                let choice = select_choice(vec!["选择招数", "道具", "逃跑"]);
                match choice {
                    0 => {
                        let choice = select_choice(vec!["正常攻击"]);
                        match choice {
                            0 => {}
                            _ => (),
                        }
                        // 开始比速度
                    }
                    1 => {
                        let choice = select_choice(vec![
                            "补血道具",
                            "补技能数量道具",
                            "捕捉道具",
                            "其他特殊道具",
                        ]);
                        match choice {
                            0 => (),
                            1 => (),
                            2 => (),
                            3 => (),
                            _ => (),
                        }
                    }
                    2 => {
                        // 暂定逃跑100% 成功
                        println!("你成功逃跑了");
                        return;
                    }
                    _ => (),
                }
                // 2 野生精灵选择招数 暂时默认选择 attack
                // 3 速度判断。
                // 如果我方选择的是道具，则我方道具先行

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
