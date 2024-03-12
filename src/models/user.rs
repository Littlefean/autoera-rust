use super::Spirit;

pub struct User {
    pub user_name: String,
    pub spirit_beg: Vec<Spirit>,
    pub spirit_beg_max: i32, // 背包目前最多可容纳多少个精灵，后续可以通过购买，换取背包容量
    money: i32,
}

impl User {
    /// 用于玩家刚开始新游戏
    pub fn player_start_init(name: String) -> User {
        User {
            money: 1000,
            user_name: name,
            spirit_beg: Vec::new(),
            spirit_beg_max: 2,
        }
    }

    // 获取精灵背包中的第一个精灵的引用
    pub fn get_first_spirit(&mut self) -> Option<&mut Spirit> {
        self.spirit_beg.first_mut()
    }
}