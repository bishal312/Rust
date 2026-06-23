fn main() {
    println!("Lifetimes function");

    //Dangaling referenging error

    // let s1: String = String::from("Bishal"); // lifetime 'b
    let s1: &str = "Bishal";

    let result: &str;

    {
        // let s2: String = String::from("Kunwar Magar"); // lifetime 'a
        let s2: &str = "Kunwar Magar";
        result = longest(&s1, &s2);
    }

    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}
