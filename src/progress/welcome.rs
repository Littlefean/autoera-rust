use crate::models::user::User;

pub fn welcome(user: &User) {
    println!("欢迎来到 auto-era {}", user.user_name);
    println!("你现在有1000元初始资金");
}
