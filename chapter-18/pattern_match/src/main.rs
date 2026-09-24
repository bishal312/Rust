#[derive(Debug)]
enum Language {
    English,
    Nepali,
    Russian,
    Hindi,
}


fn main() {
    // match pattern
    let language = Language::English;

    match language {
        Language::English => println!("Hello friend!"),
        Language::Nepali => println!("Namaste sathi!"),
        Language::Russian => println!("Previyat druk!"),
        Language::Hindi => println!("Namaste dost!"),
        lang => println!("Unsupported language! {:?}", lang)
    }

    // conditional if let expressions
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops

    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    let x = 5;

    // let Pattern = Expression;
    let (x, y, z) = (1, 2, 3);

    // Function parameters
    let point = (3, 5);
    print_coordinates(&point);

    // Irrefutable
    let x = 5; // x will always equal to 5

    //Refutable
    let x: Option<&str> = None; //x value may not be always equal
    if let Some(x) = x {
        println!("{}", x);
    };

    // can only accept irrefutable patterns:
    // function parameters
    // let statements
    // for loops

    // using @binding
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello {id: 14};

    match msg {
        Message::Hello {id: id @ 3..7} => {
            println!("Found an id in range: {id}")
        }
        Message::Hello {id: id @ 10..17} => {
            println!("Found an id in another range: {id}")
        }
        Message::Hello {id} => println!("Found some ohther id: {id}"),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
