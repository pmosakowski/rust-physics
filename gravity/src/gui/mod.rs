extern crate glium;

#[allow(dead_code)]
pub struct GliumWindow {
    display: glium::Display,
    event_loop: glium::glutin::EventsLoop,
}

impl GliumWindow {
    pub fn new(width: u32, height: u32, title: &String) -> Self {
        // glium event loop
        let event_loop = glium::glutin::EventsLoop::new();

        // create window
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title.clone())
            .with_dimensions(width, height);

        // create OpenGl context
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);

        // combine the above into an rendering target
        let display = glium::Display::new(window, context, &event_loop).unwrap();

        GliumWindow {
            display: display,
            event_loop: event_loop,
        }
    }
}
