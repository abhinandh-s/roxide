use super::config::read_config;

pub fn checks_init() {
    user_check();
    trash_dir_check();    
}

fn trash_dir_check() {
    let users = read_config("config.toml").unwrap();
    println!("{:#?}", users);
}

fn user_check() {
    
}
