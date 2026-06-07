pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> Status {
    // connect to db..
    Status::Connected
}

pub fn get_user() {
    // fetch the user from db and return
}
