mod ui;
mod functionality;
use ui::create_ui;

fn main() {
    dioxus_desktop::launch(create_ui);
}
