#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn drawSquare(){
    
}

fn main() {
    // building the display, ie. the main object
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Press S to take screenshot");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
                                 &[Vertex {
                                       position: [-0.5, -0.5],
                                       color: [0.0, 1.0, 0.0],
                                   },
                                   Vertex {
                                       position: [0.0, 0.5],
                                       color: [0.0, 0.0, 1.0],
                                   },
                                   Vertex {
                                       position: [0.5, -0.5],
                                       color: [1.0, 0.0, 0.0],
                                   }])
            .unwrap()
    };

    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110

                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec3 color;

                varying vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;

                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    )
        .unwrap();

    // drawing once

    // building the uniforms
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    event_loop.run(move |event, _, control_flow| {

        // React to events
        use glium::glutin::event::{Event, WindowEvent, ElementState, VirtualKeyCode};

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let ElementState::Pressed = input.state {
                        if let Some(VirtualKeyCode::S) = input.virtual_keycode {
                            println!("test");
                        }
                    }
                },
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer,
                  &index_buffer,
                  &program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();
    });
}
