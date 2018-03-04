extern crate conrod;
extern crate find_folder;

mod gui;
// use conrod;
// use conrod::backend::glium::glium;
// use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
// use conrod::backend::piston::event::UpdateEvent;
use gui::GliumWindow;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let title = String::from("rust-physics: gravity");

    let window = GliumWindow::new(WIDTH, HEIGHT, &title);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
}
