use sdl2::{event::Event, pixels::Color, video::GLProfile};

extern crate sdl2;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

// from docs https://docs.rs/sdl2/latest/sdl2/#
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("demo", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
