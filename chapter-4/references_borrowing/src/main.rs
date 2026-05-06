// References and borrowing

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // let x = String::from("hello");

    // change(&x); // it creates an error because it doesnot own data and cannot alter them.
    // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

    // ref
    mutable_ref();
    mutable_ref_cons();
    dangling_ref();

    //slice
    slice();
}

// The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

// fn change(some_string: &String) {  // This is an error
// some_string.push_str(", world"); // This is an error
// }

fn mutable_ref() {
    println!("<- mutable_ref function ->");

    let mut s = String::from("hello");

    change2(&mut s);
    println!("{s}");
}
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// Mutable references have one big restriction:
// If you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail

fn mutable_ref_cons() {
    println!("<- mutable_ref_cons function ->");
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // This create data race as like race condition.
    // rust compiler prevents data race.
    // println!("{r1}, {r2}")
    println!("{r1}");

    // *we can create multiple references using new scope
    // *mutable references and immutable references are not allowed together
    // *however multiple immutable references are allowed
    {
        let r2 = &mut s;
        println!("{r2}");
    }

    // ths scope explain about the uses of mut and immute ref in error free order.
    // first complete the use of immute ref and then we can declare mut ref.
    // println!, before the mutable reference is introduced
    {
        let mut s = String::from("Bishal");
        let r1 = &s;
        let r2 = &s;

        println!("{r1}, {r2}");

        let r3 = &mut s;
        println!("{r3}");
    }
}

fn dangling_ref() {
    println!("<- dangling_ref function ->");
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}")
}
// fn dangle() -> &String {
//     // error
//     let s = String::from("hello");

//     &s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!

fn no_dangle() -> String {
    let s = String::from("Magar");

    s // Ownership is moved out, and nothing is deallocated.
}

//Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

// slice

fn slice() {
    println!("<- slice function ->");

    let s = String::from("Bking");
    // let result: usize = first_word(&s);
    // println!("{result}");

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{word}");
    let sec_word = second_word(&s); // word will get the value 5
    println!("{sec_word}");

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn second_word (s: &String) -> (usize, usize) {
//     // String Slices
//     let s = String::from("nice world");
    
//     let nice = &s[0..4];
//     let world = &s[5..10];
// }

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }
    &s[..]
}