use crate::models::battle::BattleUserVsOneSpirit;
/// 初次对战的新手教学
use crate::models::user::User;
use crate::models::Spirit;

/// 选好主宠之后，立刻生成一个野生小精灵，对战一下。
pub fn battle_teach(user: User) {
    println!("直接遭遇到了一个对手");
    let mut default_spirit = Spirit::get_default_spirit();
    let mut teach_battle = BattleUserVsOneSpirit::new(user, default_spirit);
    teach_battle.battle();

    println!("对战结束了");
}
