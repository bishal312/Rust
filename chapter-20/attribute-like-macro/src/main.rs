#[route(GET, "/")]
fn index() {
    //..
}

#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // GET, "/"
    item: TokenStream // fn index() {}
) -> TokenStream {
    //..
}

fn main() {
    println!("Hello, world!");
}
