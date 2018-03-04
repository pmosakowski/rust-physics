[macro_use]
extern crate conrod;
extern crate find_folder;

// use conrod;
use conrod::backend::glium::glium;
// use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
// use conrod::backend::piston::event::UpdateEvent;

fn main() {
    let WIDTH: u32 = 800;
    let HEIGHT: u32 = 600;
    let title = "rust-physics: gravity";

    // glium event loop
    let mut events_loop = glium::glutin::EventsLoop::new();
    // create window
    let window = glium::glutin::WindowBuilder::new()
	.with_title(&title)
	.with_dimensions(WIDTH, HEIGHT);
    // create OpenGl context
    let context = glium::glutin::ContextBuilder::new()
	.with_vsync(true)
	.with_multisampling(4);
    // combine the above into an rendering target
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
}
