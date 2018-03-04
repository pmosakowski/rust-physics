extern crate conrod;
extern crate glium;
extern crate find_folder;

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
