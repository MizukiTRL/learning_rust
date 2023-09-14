#![allow(dead_code, unused_variables)]

mod database {

    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user() {}
}

pub mod auth_utils {
    pub fn login(creds: models::Credentials) {
        crate::database::get_user();
    }

    fn logout() {}

    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}

use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {

    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
