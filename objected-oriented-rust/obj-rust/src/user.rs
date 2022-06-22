#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub phone_number: String,
}

impl User {
    pub fn new_user(username: String, email: String, phone_number: String) -> User {
        println!("adding user");
        User {
            username,
            email,
            phone_number,
        }
    }
    pub fn change_number(&mut self, phone_number: String) {
        self.phone_number = phone_number;
        println!("phone_number changed");
    }
}
