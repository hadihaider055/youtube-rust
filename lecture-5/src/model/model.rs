#![allow(dead_code)]

pub mod Models {
    use crate::Status;
    // eslint-disable
    // @ts-ignore

    pub struct User {
        pub name: String,
        pub username: String,
        pub password: String,
        pub age: i32,
        pub is_verified: Status,
    }

    pub struct Credentials {
        pub username: String,
        pub password: String,
        pub status: Status,
    }
}
