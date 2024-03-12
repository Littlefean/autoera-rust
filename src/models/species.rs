/**
 * 通常来说
 *
 * 火系：攻击力高但血量少
 * 水系：均衡
 * 草系：血量多
 *
 * 飞行：速度快 攻击力不高
 * 电系：速度快 攻击力高 血少 防御低
 * 地系：防御力高
 */
pub enum SpeciesType {
    Normal = 0,
    Hydro = 1,
    Flare = 2,
    Floral = 3,
    Electra = 4,
    Terra = 5,
    Sky = 6,
}

impl SpeciesType {
    pub fn emoji(&self) -> String {
        match self {
            SpeciesType::Normal => String::from("⭐"),
            SpeciesType::Hydro => String::from("💧"),
            SpeciesType::Flare => String::from("🔥"),
            SpeciesType::Floral => String::from("🌿"),
            SpeciesType::Sky => String::from("🪽"),
            SpeciesType::Electra => String::from("⚡"),
            SpeciesType::Terra => String::from("🪨"),
            _ => String::from("❔"),
        }
    }
}

/**
 * 判断一个系别攻击另一个系别时的伤害系数
 * 1.0 代表 正常伤害
 * 0.5 代表 一半伤害
 * 2.0 代表 属性克制，二倍伤害，例如 火系打草系
 */
pub fn get_attack_rate(attack_species: SpeciesType, defense_species: SpeciesType) -> f32 {
    // 一个二维数组来表示属性相克表
    let effectiveness_chart: [[f32; 7]; 7] = [
        //      Normal Hydro  Flare Floral Electra Terra  Sky
        /*Normal*/
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        /*Hydro*/ [1.0, 1.0, 2.0, 0.5, 1.0, 2.0, 1.0],
        /*Flare*/ [1.0, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0],
        /*Floral*/ [1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0],
        /*Electra*/ [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        /*Terra*/ [1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0],
        /*Sky*/ [1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0],
    ];

    // 使用攻击系别和防御系别的值作为索引来获取伤害系数
    let attack_index = attack_species as usize;
    let defense_index = defense_species as usize;

    effectiveness_chart[attack_index][defense_index]
}
