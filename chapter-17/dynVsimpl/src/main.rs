trait Render {
    fn render(&self);
}

struct Button;
struct Image;

impl Render for Button {
    fn render(&self) {
        println!("Button");
    }
}

impl Render for Image {
    fn render(&self) {
        println!("Image");
    }
}

fn render_all (items: &[&dyn Render]) {
    for item in items {
        item.render();
    }
}

fn main() {
    println!("&dyn vs &impl");

    let btn: &Button = &Button;
    let img: &Image = &Image;

    let items: Vec<&dyn Render> = vec![img, btn];

    render_all(&items);
}
