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


// function-like macros

let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    //..
}