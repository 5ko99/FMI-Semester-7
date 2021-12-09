pub struct ProductUser {
    username: String,
    email: String,
}

pub fn new(username: String, email: String) -> ProductUser {
    ProductUser { username, email }
}

impl ProductUser {
    pub fn get_username(self) -> String {
        self.username
    }
}
