use crate::models::user::User;
use crate::tools::cmd::clear_console;

pub fn welcome(user: &User) {
    clear_console();

    println!("");
    println!("                       d8                                       ");
    println!("    /~~~8e  888  888 _d88__  e88~-_   e88~~8e  888-~\\   /~~~8e  ");
    println!("        88b 888  888  888   d888   i d888  88b 888          88b ");
    println!("   e88~-888 888  888  888   8888   | 8888__888 888     e88~-888 ");
    println!("  C888  888 888  888  888   Y888   ' Y888    , 888    C888  888 ");
    println!("   \"88_-888 \"88_-888  \"88_/  \"88_-~   \"88___/  888     \"88_-888 ");

    println!("欢迎来到 auto-era,{}!", user.user_name);
    println!("你现在有1000元初始资金");
}
