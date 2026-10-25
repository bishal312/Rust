

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let a = add_twice(add_one, 12);
    println!("{}",a);

    let list_of_num = vec![1,2,3];
    let list_of_strings: Vec<String> = 
        // list_of_num.iter().map(|i| i.to_string()).collect();
        list_of_num.iter().map(ToString::to_string).collect(); //function pointer ToString::to_string;
    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
    
    println!("{:?}", list_of_statuses);
}
