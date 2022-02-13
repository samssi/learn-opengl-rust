use glfw::{Context, Glfw, Window, WindowEvent};
use gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use crate::processor::process_events;

// settings
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TRIANGLE_VERTICES: [f32; 9] =
    [-0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0, 0.5, 0.0
    ];

const VERTEX_SHADER_GLSL: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_GLSL: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn glsl_as_cstring(glsl_source: &str) -> CString {
    return CString::new(glsl_source.as_bytes()).unwrap();
}


fn initialize_glfw() -> Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    return glfw;
}

fn create_glfw_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    let (window, events) =
        glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
    return (window, events);
}

fn render(mut window: Window, events: Receiver<(f64, WindowEvent)>, mut glfw: Glfw) {
    let (shader_program, vao) = unsafe {
        let vertex_shader = compile_shader(gl::CreateShader(gl::VERTEX_SHADER), VERTEX_SHADER_GLSL);
        let fragment_shader = compile_shader(gl::CreateShader(gl::FRAGMENT_SHADER), FRAGMENT_SHADER_GLSL);
        let shader_program = link_shader(vertex_shader, fragment_shader);

        (shader_program, create_vao())
    };

    while !window.should_close() {
        process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3 , 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // 2. use our shader program when we want to render an object
            // draw our first triangle
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            // 3. now draw the object
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        window.swap_buffers();
        glfw.poll_events();
    }

}

fn compile_shader(shader: GLuint, shader_source_code: &str) -> GLuint {
    let c_string_shader = glsl_as_cstring(shader_source_code);
    unsafe {
        gl::ShaderSource(shader, 1, &c_string_shader.as_ptr(), ptr::null());
        gl::CompileShader(shader);
    }

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    unsafe {
        info_log.set_len(512-1);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            // TODO: error should display the shader type
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }
    }

    return shader;
}

fn link_shader(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);


        // TODO: dry
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        return shader_program;
    }
}

fn create_vao() -> (GLuint) {
    let (mut vbo, mut vao) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        // copy our vertices array in a buffer for OpenGL to use
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (TRIANGLE_VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &TRIANGLE_VERTICES[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        // then set the vertex attributes pointers
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the
        // vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    return vao;
}

pub fn hello() {
    let glfw = initialize_glfw();
    let (mut window, events) = create_glfw_window(&glfw);

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    render(window, events, glfw);
}