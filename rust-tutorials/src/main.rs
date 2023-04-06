use sdl2::{
    event::Event,
    video::{GLProfile, Window},
    EventPump,
};
use std::{
    ffi::CString,
    fs::File,
    io::Read,
    mem::{size_of, size_of_val},
};

extern crate gl;
extern crate sdl2;

use gl::types::{GLchar, GLfloat, GLsizeiptr, GLuint, GLvoid};

pub mod render_gl;

// from docs https://docs.rs/sdl2/latest/sdl2/#
fn main() {
    // CONFIGURE SDL2 & OPENGL
    // =======================
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("demo", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl_context.event_pump().unwrap();

    create_shader_program(&mut event_pump, window)
}

fn create_shader_program(event_pump: &mut EventPump, window: Window) {
    // SHADER PROGRAM
    // ==============
    let vertex_source = read_source_from_file("shader.vert");
    let fragment_source = read_source_from_file("shader.frag");

    unsafe {
        // vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        let string = String::from_iter(vec![' '; 512]);
        let info_log = CString::new(string).unwrap();

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_ptr() as *mut GLchar,
            );
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{:?}", info_log);
            println!("Shader: {:?}", vertex_source);
        }

        // fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{:?}",
                info_log
            );
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_ptr() as *mut GLchar,
            );
            println!("ERROR::PROGRAM::LINKING_FAILED\n{:?}", info_log);
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // set up vertices
        type Vertex<T = GLfloat> = [T; 3];
        let vertices: Vec<Vertex> = vec![
            [0.5, 0.5, 0.0],
            [0.5, -0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ];

        let indices: Vec<GLuint> = vec![0, 1, 3, 1, 2, 3];

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        // https://learnopengl.com/Getting-started/Hello-Triangle
        // let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        gl::GenVertexArrays(1, &mut vao);
        // triangle
        gl::GenBuffers(1, &mut vbo);
        // rectangle
        gl::GenBuffers(1, &mut ebo);
        gl::BindVertexArray(vao);

        // VERTEX BUFFER OBJECT -----------
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // ELEMENT BUFFER OBJECT -----------
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        // VERTEX ATTRIB ARRAY ----------
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (size_of::<Vertex>()).try_into().unwrap(),
            0 as *const GLvoid,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);

            window.gl_swap_window();
        }
    }
}

fn read_source_from_file(file_name: &str) -> CString {
    let mut source_file = File::open(format!("src/{file_name}")).unwrap();
    let mut contents = String::new();

    source_file.read_to_string(&mut contents).unwrap();

    CString::new(contents).unwrap()
}

fn _check_gl_error() {
    let error = unsafe { gl::GetError() };

    if error != 0 {
        println!("GL::ERROR\n{:?}", error);
    }
}
