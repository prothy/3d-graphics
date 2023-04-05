use sdl2::{event::Event, video::GLProfile};
use std::{
    ffi::{c_char, CString},
    mem,
};

extern crate gl;
extern crate sdl2;

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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // SHADER PROGRAM
    // ==============
    let vertex_source = CString::new("shader.vert").unwrap();
    let fragment_source = CString::new("shader.frag").unwrap();

    unsafe {
        // vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        let info_log = [' ' as c_char; 512].as_mut_ptr();

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), info_log);
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{:?}", info_log);
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
            gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), info_log);
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{:?}",
                info_log
            );
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log);
            println!("ERROR::PROGRAM::LINKING_FAILED\n{:?}", info_log);
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let vertices: Vec<f32> = vec![
            0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
        ];
        let indices: Vec<i8> = vec![0, 1, 3, 1, 2, 3];

        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        gl::UseProgram(shader_program);

        // https://learnopengl.com/Getting-started/Hello-Triangle
        // let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        gl::GenVertexArrays(1, &mut vao);
        // triangle
        gl::GenBuffers(1, &mut vbo);
        // rectangle
        gl::GenBuffers(1, &mut ebo);
        gl::BindVertexArray(vao);

        // triangle
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<f32>()).try_into().unwrap(),
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<i8>()).try_into().unwrap(),
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        let _error = gl::GetError();

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * mem::size_of::<f32>()).try_into().unwrap(),
            std::ptr::null(),
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
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const std::os::raw::c_void,
            );
        }

        window.gl_swap_window();
    }
}
