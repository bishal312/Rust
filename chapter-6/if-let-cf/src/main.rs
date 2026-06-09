
fn if_let(){
    let config_max: Option<u8> = Some(3_u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn main() {
    if_let();
}
