use glfw::{Context, Key, Action, Window, WindowEvent, Glfw};
use std::sync::mpsc::Receiver;
use gl::types::*;

// settings
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TRIANGLE_VERTICES: [f32; 9] =
    [-0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0, 0.5, 0.0
    ];


fn initialize_glfw() -> Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    return glfw;
}

fn create_glfw_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    let (mut window, events) =
        glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
    return (window, events);
}

fn render(mut window: Window, events: Receiver<(f64, WindowEvent)>, mut glfw: Glfw) {
    while !window.should_close() {
        // events
        process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3 , 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }

}

pub fn hello() {
    let mut glfw = initialize_glfw();
    let (mut window, events) = create_glfw_window(&glfw);

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    render(window, events, glfw);
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}