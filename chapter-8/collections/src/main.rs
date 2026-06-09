fn main() {
    println!("Collections");

    let mut vec: Vec<i32> = vec![1, 2, 3];

    // let mut vec: Vec<i32> = Vec::new();

    //update
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    //shadowing for immutation
    let vec = vec;

    //error because of immutable
    // vec.push(33);

    //read
    let third: &i32 = &vec[3];
    println!("The value of vec is {:?}", vec);
    println!("Third > {:?}", third);

    let fourth: &i32 = match vec.get(30) {
        Some(value) => value,
        None => {
            println!("The given index is out of vector list");
            &-1
        }
    };

    println!("Fourth number in vector {:?}", fourth);

    // dereference operator *
    let mut v = vec![55, 66, 100];
    println!("{:?}", v);
    for i in &mut v {
        //& reference operator
        *i *= 2; // * deference operator
    }
    println!("{:?}", v);

    mixtype();
}
#[derive(Debug)]
enum Login {
    username(String),
    password(String),
    email(String),
}
fn mixtype() {
    let user1: Vec<Login> = vec![
        Login::username(String::from("Bishal")),
        Login::password(String::from("bking1212")),
        Login::email(String::from("bishal12@gmail.com")),
    ];

    println!("{:?}", user1);
}
