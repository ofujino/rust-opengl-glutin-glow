use glow::*;

mod matrix_state;
use matrix_state::MatrixState;

struct GLMatrixState {
    world_matrix: gl_matrix::common::Mat4,
    view_matrix: gl_matrix::common::Mat4,
    proj_matrix: gl_matrix::common::Mat4,
}

impl MatrixState for GLMatrixState {
    fn new(width: u32, height: u32) -> Self {
        println!("use gl_matrix");

        let world_matrix: gl_matrix::common::Mat4 = [0.0; 16];
        let mut view_matrix: gl_matrix::common::Mat4 = [0.0; 16];
        let mut proj_matrix: gl_matrix::common::Mat4 = [0.0; 16];

        gl_matrix::mat4::perspective(
            &mut proj_matrix,
            gl_matrix::common::to_radian(45.0),
            width as f32 / height as f32,
            0.1,
            Some(100.0),
        );

        gl_matrix::mat4::look_at(
            &mut view_matrix,
            &gl_matrix::vec3::from_values(0.0, 0.0, 5.0),
            &gl_matrix::vec3::from_values(0.0, 0.0, 0.0),
            &gl_matrix::vec3::from_values(0.0, 1.0, 0.0),
        );

        Self {
            world_matrix,
            view_matrix,
            proj_matrix,
        }
    }

    fn update(&mut self, step: f32) {
        gl_matrix::mat4::identity(&mut self.world_matrix);

        let tmp = gl_matrix::mat4::clone(&self.world_matrix);

        gl_matrix::mat4::rotate(
            &mut self.world_matrix,
            &tmp,
            step,
            &gl_matrix::vec3::from_values(0.0, 1.0, 0.0),
        );
    }

    fn get_world(&mut self) -> &[f32] {
        return &self.world_matrix;
    }

    fn get_view(&mut self) -> &[f32] {
        return &self.view_matrix;
    }

    fn get_projection(&mut self) -> &[f32] {
        return &self.proj_matrix;
    }
}

struct GlamState {
    world_matrix: glam::Mat4,
    view_matrix: glam::Mat4,
    proj_matrix: glam::Mat4,
    //array: [f32; 16],
}

impl MatrixState for GlamState {
    fn new(width: u32, height: u32) -> Self {
        println!("use glam");

        let world_matrix = glam::Mat4::IDENTITY;

        let view_matrix = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0, 0.0, 5.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
        );

        let proj_matrix = glam::Mat4::perspective_rh(
            45.0f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            100.0,
        );

        Self {
            world_matrix,
            view_matrix,
            proj_matrix,
            //array: [0.0; 16],
        }
    }

    fn update(&mut self, step: f32) {
        //self.world_matrix = glam::Mat4::IDENTITY;
        //self.world_matrix *= glam::Mat4::from_rotation_y(step);

        self.world_matrix = glam::Mat4::from_rotation_y(step);
    }

    fn get_world(&mut self) -> &[f32] {
        //self.array = self.world_matrix.to_cols_array();
        //return &self.array;
        return self.world_matrix.as_ref();
    }

    fn get_view(&mut self) -> &[f32] {
        //self.array = self.view_matrix.to_cols_array();
        //return &self.array;
        return self.view_matrix.as_ref();
    }

    fn get_projection(&mut self) -> &[f32] {
        //self.array = self.proj_matrix.to_cols_array();
        //return &self.array;
        return self.proj_matrix.as_ref();
    }
}

struct GlmState {
    world_matrix: glm::Mat4,
    view_matrix: glm::Mat4,
    proj_matrix: glm::Mat4,
    array: [f32; 16],
}

impl MatrixState for GlmState {
    fn new(width: u32, height: u32) -> Self {
        println!("use glm");

        let world_matrix = glm::Mat4::new(
            glm::vec4(1.0, 0.0, 0.0, 0.0),
            glm::vec4(0.0, 1.0, 0.0, 0.0),
            glm::vec4(0.0, 0.0, 1.0, 0.0),
            glm::vec4(0.0, 0.0, 0.0, 1.0),
        );

        let view_matrix = glm::ext::look_at(
            glm::vec3(0.0, 0.0, 5.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0),
        );

        let proj_matrix = glm::ext::perspective(
            45.0f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            100.0,
        );

        Self {
            world_matrix,
            view_matrix,
            proj_matrix,
            array: [0.0; 16],
        }
    }

    fn update(&mut self, step: f32) {
        #[rustfmt::skip]
        let identity_matrix = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        self.world_matrix = glm::ext::rotate(&identity_matrix, step, glm::vec3(0.0, 1.0, 0.0));
    }

    fn get_world(&mut self) -> &[f32] {
        let mut i = 0;
        for a in self.world_matrix.as_array() {
            for v in a.as_array() {
                self.array[i] = *v;
                i += 1;
            }
        }
        return &self.array;
    }

    fn get_view(&mut self) -> &[f32] {
        let mut i = 0;
        for a in self.view_matrix.as_array() {
            for v in a.as_array() {
                self.array[i] = *v;
                i += 1;
            }
        }
        return &self.array;
    }

    fn get_projection(&mut self) -> &[f32] {
        let mut i = 0;
        for a in self.proj_matrix.as_array() {
            for v in a.as_array() {
                self.array[i] = *v;
                i += 1;
            }
        }
        return &self.array;
    }
}

struct CGMathState {
    world_matrix: cgmath::Matrix4<f32>,
    view_matrix: cgmath::Matrix4<f32>,
    proj_matrix: cgmath::Matrix4<f32>,
}

impl MatrixState for CGMathState {
    fn new(width: u32, height: u32) -> Self {
        println!("use cgmath");

        /*
        #[rustfmt::skip]
        let world_matrix = cgmath::Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        }
        */

        #[rustfmt::skip]
        let world_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
            cgmath::Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
            cgmath::Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
            cgmath::Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
            cgmath::Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        );

        #[rustfmt::skip]
        let view_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::look_at(
            cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            /*
            cgmath::Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            */
            cgmath::Vector3::unit_y(),
        );

        #[rustfmt::skip]
        let proj_matrix: cgmath::Matrix4<f32> = cgmath::perspective(
            cgmath::Deg(45.0f32),
            width as f32 / height as f32,
            0.1,
            100.0
        );

        Self {
            world_matrix,
            view_matrix,
            proj_matrix,
        }
    }

    fn update(&mut self, step: f32) {
        //self.world_matrix = cgmath::Matrix4::from_angle_y(cgmath::Rad(step));

        let mut world_matrix = cgmath::SquareMatrix::identity();

        world_matrix = world_matrix * cgmath::Matrix4::from_angle_y(cgmath::Rad(step));

        self.world_matrix = world_matrix;
    }

    fn get_world(&mut self) -> &[f32] {
        let array: &[f32; 16] = self.world_matrix.as_ref();
        return array;
    }

    fn get_view(&mut self) -> &[f32] {
        let array: &[f32; 16] = self.view_matrix.as_ref();
        return array;
    }

    fn get_projection(&mut self) -> &[f32] {
        let array: &[f32; 16] = self.proj_matrix.as_ref();
        return array;
    }
}

fn main() {
    let width = 640;
    let height = 480;

    #[cfg(feature = "gl_matrix")]
    let mut state = GLMatrixState::new(width, height);

    #[cfg(feature = "glam")]
    let mut state = GlamState::new(width, height);

    #[cfg(feature = "glm")]
    let mut state = GlmState::new(width, height);

    #[cfg(feature = "cgmath")]
    let mut state = CGMathState::new(width, height);

    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

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

        let mut frame: f32 = 1.0;

        event_loop.run(move |event, _, control_flow| match event {
            glutin::event::Event::MainEventsCleared => {
                context.window().request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                state.update(frame / 20.0);

                gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                gl.bind_vertex_array(Some(vertex_array));
                gl.use_program(Some(program));

                gl.uniform_matrix_4_f32_slice(
                    gl.get_uniform_location(program, "uMMatrix").as_ref(),
                    false,
                    state.get_world(),
                );

                gl.uniform_matrix_4_f32_slice(
                    gl.get_uniform_location(program, "uVMatrix").as_ref(),
                    false,
                    state.get_view(),
                );

                gl.uniform_matrix_4_f32_slice(
                    gl.get_uniform_location(program, "uPMatrix").as_ref(),
                    false,
                    state.get_projection(),
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
