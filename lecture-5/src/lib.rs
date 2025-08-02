mod model;

pub enum Status {
    TODO,
    INPROGRESS,
    DONE,
}

mod Controllers {
    pub fn login() {}

    use crate::Status;
    use crate::model::model::Models::{Credentials, User};

    pub fn signup() {
        let user = User {
            name: String::from("Hello"),
            username: String::from("Hello"),
            password: String::from("Hello"),
            age: 20,
            is_verified: Status::DONE,
        };
        let creds = Credentials {
            username: String::from("Hello"),
            password: String::from("Hello"),
            status: Status::DONE,
        };
    }

    fn logout() {}
}
