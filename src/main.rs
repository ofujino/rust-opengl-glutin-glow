use gl_matrix::common::*;
use gl_matrix::{mat4, vec3};
use glow::*;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(640, 480));

        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            // see: http://michaelshaw.io/rust-game-24h-talk/talk.html#/20
            //.with_gl_profile(glutin::GlProfile::Core)
            //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl = glow::Context::from_loader_function(|s| context.get_proc_address(s) as *const _);

        let program = create_shader_program(
            &gl,
            &include_str!("shader.vert"),
            &include_str!("shader.frag"),
        );

        let vertex_array = gl.create_vertex_array().unwrap();

        gl.clear_color(0.2, 0.2, 0.2, 1.0);

        let mut world_matrix: Mat4 = [0.; 16];
        let mut view_matrix: Mat4 = [0.; 16];
        let mut proj_matrix: Mat4 = [0.; 16];

        //mat4::identity(&mut view_matrix);
        //mat4::identity(&mut proj_matrix);

        let eye = vec3::from_values(0., 0., 5.);
        let center = vec3::from_values(0., 0., 0.);
        let up = vec3::from_values(0., 1., 0.);
        let aspect = context.window().inner_size().width as f32
            / context.window().inner_size().height as f32;

        mat4::look_at(&mut view_matrix, &eye, &center, &up);
        mat4::perspective(&mut proj_matrix, to_radian(45.), aspect, 0.1, Some(100.0));

        let mut frame: f32 = 1.0;

        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::MainEventsCleared => {
                    context.window().request_redraw();
                }
                glutin::event::Event::RedrawRequested(_) => {
                    mat4::identity(&mut world_matrix);
                    let tmp = mat4::clone(&world_matrix);
                    mat4::rotate(
                        &mut world_matrix,
                        &tmp,
                        frame / 20.0,
                        &vec3::from_values(0., 1., 0.),
                    );

                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    gl.bind_vertex_array(Some(vertex_array));
                    gl.use_program(Some(program));
                    gl.uniform_matrix_4_f32_slice(
                        gl.get_uniform_location(program, "uMMatrix").as_ref(),
                        false,
                        &world_matrix,
                    );
                    gl.uniform_matrix_4_f32_slice(
                        gl.get_uniform_location(program, "uVMatrix").as_ref(),
                        false,
                        &view_matrix,
                    );
                    gl.uniform_matrix_4_f32_slice(
                        gl.get_uniform_location(program, "uPMatrix").as_ref(),
                        false,
                        &proj_matrix,
                    );
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    gl.use_program(None);
                    gl.bind_vertex_array(None);
                    context.swap_buffers().unwrap();
                    frame += 1.0;
                }
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key_code) = input.virtual_keycode {
                            if input.state == glutin::event::ElementState::Pressed {
                                match key_code {
                                    glutin::event::VirtualKeyCode::Escape => {
                                        control_flow.set_exit();
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

fn create_shader_program(gl: &glow::Context, vs: &str, fs: &str) -> glow::NativeProgram {
    unsafe {
        let program = gl.create_program().unwrap();

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, vs);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("{}", gl.get_shader_info_log(vertex_shader));
        }

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, fs);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader) {
            panic!("{}", gl.get_shader_info_log(fragment_shader));
        }

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        gl.detach_shader(program, vertex_shader);
        gl.detach_shader(program, fragment_shader);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        return program;
    }
}
