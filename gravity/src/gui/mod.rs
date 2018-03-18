extern crate conrod;
extern crate glium;
extern crate find_folder;

use std;

#[allow(dead_code)]
struct EventLoop<'a> {
    events_loop: &'a mut glium::glutin::EventsLoop,
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl<'a> EventLoop<'a> {
    pub fn new(events_loop: &'a mut glium::glutin::EventsLoop) -> Self {
        EventLoop {
            events_loop: events_loop,
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        self.events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            self.events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}


#[allow(dead_code)]
pub struct GliumWindow {
    width: u32,
    height: u32,
    title: String,
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
            width: width,
            height: height,
            title: title.clone(),
            display: display,
            event_loop: event_loop,
        }
    }
}

#[allow(dead_code)]
pub struct ConrodGUI {
    ui: conrod::Ui,
}

impl ConrodGUI {
    pub fn new(width: u32, height: u32) -> Self {
        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        ConrodGUI::load_font(&mut ui);

        ConrodGUI {
            ui: ui,
        }
    }

    fn load_font(ui: &mut conrod::Ui) {
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(&font_path).unwrap();
        println!("Loaded font {:?}", &font_path);
    }
}
