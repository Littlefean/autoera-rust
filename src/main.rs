mod models;
mod progress;
mod tools;

fn main() {
    let user_name = tools::cmd::input("请给你自己起一个游戏名:".to_owned());

    let mut user_data = models::user::User::player_start_init(user_name);
    progress::welcome::welcome(&user_data);

    let user_primary_spirit = progress::primary_choice::progress_choice_primary_spirit();
    user_data.spirit_beg.add_spirit(user_primary_spirit);

    progress::battle_teach::battle_teach(user_data);
}
