pub mod database;

pub mod auth_utils;

pub mod utils;

use auth_utils::login;
use database::{connect_to_database, Status};

pub fn authenticate(cred: Credentials) {
    if let database::Status::Connected = connect_to_database() {}
}
