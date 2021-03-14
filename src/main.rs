#[macro_use]
extern crate glium;
extern crate image;

mod lm;
mod tetris;

use crate::lm::Mat4;
use crate::tetris::PlaySpace;
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    vec_color: (f32, f32, f32),
}

fn main() {
    // setup glium
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    // do one time stuff

    implement_vertex!(Vertex, position, vec_color);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec3 vec_color;
        out vec3 my_color;
        uniform mat4 matrix;
        void main() {
            my_color = vec_color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 my_color;
        out vec4 color;
        void main() {
            color = vec4(my_color, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut main_play_space = PlaySpace::initialize();

    // Real(tm) render loop
    events_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        main_play_space.tick(tetris::Action::None);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // draw main space
        let points = vec![
            [-0.5f32, 1.0],
            [-0.5, -1.0],
            [0.5, 1.0],
            [-0.5, -1.0],
            [0.5, 1.0],
            [0.5, -1.0],
        ];
        let points_proper = points_to_points_proper(points, main_play_space.color);

        let uniforms = uniform! {
            matrix: Mat4::identity().matrix,
        };

        let vertex_buffer = glium::VertexBuffer::new(&display, &points_proper).unwrap();
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        // draw settled blocks
        let mut vertices: Vec<Vertex> = Vec::new();
        let simple_space = main_play_space.space_with_falling_as_settled();
        for i in 0..simple_space.len() {
            for j in 0..simple_space[i].len() {
                if let tetris::SpaceState::SettledTetromino(color) = simple_space[i][j] {
                    let points = vec![
                        [i as f32 - 0.5, j as f32 - 0.5],
                        [i as f32 - 0.5, j as f32 + 0.5],
                        [i as f32 + 0.5, j as f32 + 0.5],
                        [i as f32 - 0.5, j as f32 - 0.5],
                        [i as f32 + 0.5, j as f32 + 0.5],
                        [i as f32 + 0.5, j as f32 - 0.5],
                    ];
                    let mut points_proper = points_to_points_proper(points, color);
                    vertices.append(&mut points_proper);
                }
            }
        }
        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
        let uniforms = uniform! {
            matrix: Mat4::identity().scale_by(0.1, 0.1, 1.0).translate_by(-0.5, -0.95, 0.0).matrix,
        };
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    });
}

fn points_to_points_proper(points: Vec<[f32; 2]>, color: (f32, f32, f32)) -> Vec<Vertex> {
    let mut points_proper: Vec<Vertex> = Vec::new();
    for point in points {
        points_proper.push(Vertex {
            position: point,
            vec_color: color,
        });
    }
    return points_proper;
}
