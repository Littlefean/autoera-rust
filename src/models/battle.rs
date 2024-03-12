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
    pub fn battle(&mut self) {
        // 假设我们是后手
        loop {
            if let Some(first_spirit) = self.user.get_first_spirit() {
                // 使用 first_spirit
                // 假设对手先打我们
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
