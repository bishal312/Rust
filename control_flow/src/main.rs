fn if_else() {
    let x: u32 = 15;
    if x < 20 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    // let container = if container {"Bsal"} else {21}.  Invalid statement (mismatch type)
    let container = if condition { "Bsal" } else { "Kunwar" };

    println!("The value of container is: {container}");
}

fn _loop() {
    // loop {
    //     println!("Again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        // here 'counting_up is loop label which starts with ' single quote.
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println! {"remaining = {remaining}"};
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn _while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // when index is greater than the actual total items it will give error.
        //better to use for because it is error-prone; so we use for loop.
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn _for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..5).rev() { //range with rev methode
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    if_else();
    _loop();
    _while();
    _for();
}
