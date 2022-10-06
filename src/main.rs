use gl_matrix::common::*;
use gl_matrix::{mat4, vec3};
use glow::*;

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(640, 480));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            // see: http://michaelshaw.io/rust-game-24h-talk/talk.html#/20
            //.with_gl_profile(glutin::GlProfile::Core)
            //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        let program = gl.create_program().unwrap();

        let vertex_array = gl.create_vertex_array().unwrap();

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        let vertex_shader_source = r#"#version 330
        const vec2 positions[3] = vec2[3](
            vec2( 0.0f,  1.0f),
            vec2(-1.0f, -1.0f),
            vec2( 1.0f, -1.0f)
        );
        const vec3 colors[3] = vec3[3](
            vec3(1, 0, 0),
            vec3(0, 1, 0),
            vec3(0, 0, 1)
        );
        out vec4 vColor;
        uniform mat4 uMMatrix;
        uniform mat4 uVMatrix;
        uniform mat4 uPMatrix;
        void main() {
            gl_Position = uPMatrix * uVMatrix * uMMatrix *vec4(positions[gl_VertexID % 3], 0.0, 1.0);
            vColor = vec4(colors[gl_VertexID % 3], 1.0);
        }"#;
        gl.shader_source(vertex_shader, vertex_shader_source);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("{}", gl.get_shader_info_log(vertex_shader));
        }

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        let fragment_shader_source = r#"#version 330
        precision mediump float;
        in vec4 vColor;
        out vec4 outColor;
        void main() {
            outColor = vColor;
        }"#;
        gl.shader_source(fragment_shader, fragment_shader_source);
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

        gl.clear_color(0.2, 0.2, 0.2, 1.0);

        let mut frame: f32 = 1.0;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();
            match event {
                glutin::event::Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                glutin::event::Event::RedrawRequested(_) => {
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    gl.bind_vertex_array(Some(vertex_array));
                    gl.use_program(Some(program));
                    let mut world_matrix: Mat4 = [0.; 16];
                    let mut view_matrix: Mat4 = [0.; 16];
                    let mut proj_matrix: Mat4 = [0.; 16];
                    mat4::identity(&mut world_matrix);
                    mat4::identity(&mut view_matrix);
                    mat4::identity(&mut proj_matrix);
                    let eye = vec3::from_values(0., 0., 5.);
                    let center = vec3::from_values(0., 0., 0.);
                    let up = vec3::from_values(0., 1., 0.);
                    let m2 = mat4::clone(&world_matrix);
                    mat4::rotate(
                        &mut world_matrix,
                        &m2,
                        frame / 20.0,
                        &[0.0f32, 1.0f32, 0.0f32],
                    );
                    mat4::look_at(&mut view_matrix, &eye, &center, &up);
                    mat4::perspective(
                        &mut proj_matrix,
                        to_radian(45.),
                        window.window().inner_size().width as f32 / window.window().inner_size().height as f32,
                        0.1,
                        Some(100.0),
                    );
                    let l1 = gl.get_uniform_location(program, "uMMatrix");
                    gl.uniform_matrix_4_f32_slice(l1.as_ref(), false, &world_matrix);
                    let l2 = gl.get_uniform_location(program, "uVMatrix");
                    gl.uniform_matrix_4_f32_slice(l2.as_ref(), false, &view_matrix);
                    let l3 = gl.get_uniform_location(program, "uPMatrix");
                    gl.uniform_matrix_4_f32_slice(l3.as_ref(), false, &proj_matrix);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    gl.use_program(None);
                    gl.bind_vertex_array(None);
                    window.swap_buffers().unwrap();
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
