mod models;
mod progress;
mod tools;

fn main() {
    let user_name = tools::cmd::input(String::from("请给你自己起一个游戏名:"));

    let mut user_data = models::user::User::player_start_init(user_name);
    progress::welcome::welcome(&user_data);

    let user_primary_spirit = progress::primary_choice::progress_choice_primary_spirit();
    user_data.spirit_beg.push(user_primary_spirit);

    progress::battle_teach::battle_teach(user_data);
}
