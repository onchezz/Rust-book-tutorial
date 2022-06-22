use user::User;
mod persons;
pub mod user;

#[derive(Debug)]
pub struct Users {
    users: Vec<User>,
}
impl Users {
    pub fn new_list_of_users() -> Self {
        Self { users: Vec::new() }
    }
    pub fn add_users(&mut self, user: User) {
        self.users.push(user)
    }
    pub fn print_all(&self) {
        for user in self.users.iter() {
            println!("this users : {:#?}", user)
        }
    }
}

pub mod others {

    pub fn prints_others() {
        println!("printing others !!!");
    }
}
