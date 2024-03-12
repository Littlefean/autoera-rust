/// 精灵背包的实现
use super::spirit::Spirit;

pub struct SpiritBeg {
    content: Vec<Spirit>,
    max_size: usize,
}

impl SpiritBeg {
    // 创建一个空背包
    pub fn new() -> Self {
        SpiritBeg {
            content: Vec::new(),
            max_size: 2,
        }
    }
    /// 获取背包的最大容量
    pub fn get_max_size(&self) -> usize {
        return self.max_size;
    }

    /// 将精灵新加入背包
    pub fn add_spirit(&mut self, spirit: Spirit) -> bool {
        if self.content.len() >= self.max_size {
            return false;
        }
        self.content.push(spirit);
        return true;
    }

    /// 获得背包中第一个上场的宠物
    pub fn get_primary_spirit(&mut self) -> Option<&mut Spirit> {
        return self.content.first_mut();
    }
}
