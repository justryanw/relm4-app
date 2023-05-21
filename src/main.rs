use relm4::gtk;

enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

struct AppWidgets {
    label: gtk::Label
}

fn main() {
    println!("Hello, world!");
}
