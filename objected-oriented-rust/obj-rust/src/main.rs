use obj_rust::{user::User, *};

fn main() {
    let mut users = Users::new_list_of_users();
    let user1 = User::new_user(
        "rusty".to_string(),
        "ad@gmail.com".to_string(),
        "123456789".to_string(),
    );
    users.add_users(user1);
    let mut user2 = User::new_user(
        "tutorial".to_string(),
        "cdf@gmail.com".to_string(),
        "128748476789".to_string(),
    );
    {
        user2.change_number("2363237".to_string());
    }
    users.add_users(user2);

    users.print_all();
    others::prints_others();
}
