// References and borrowing

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let x = String::from("hello");
    change(&x); // it creates an error because it doesnot own data and cannot alter them.
    // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
}

// The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

  fn change(some_string: &String) {
    some_string.push_str(", world");
}


fn mutable_ref() {
    let mut s = String::from("hello");

    change2(&mut s);
}
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}