#[macro_use]
extern crate glium;

mod squares;

const INDICES: [u16; 6] = [0, 1, 3, 0, 2, 3];

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        .with_title("lifegame");

    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        uniform float t;

        void main() {
           vec2 pos = position;
           gl_Position = vec4(pos, 0.0, t);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;

        void main() {
            color = vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut position_x = 0.0;
    let mut position_y = 0.0;

    let mut fields = [false; 20 * 20];

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for y in 0..20 {
            for x in 0..20 {
                let square = get_square(x, y);
                let vertex_buffer = glium::VertexBuffer::new(&display, &square).unwrap();
                target.draw(&vertex_buffer,
                            &indices,
                            &program, 
                            &uniform!{t: if fields[x + y * 20] { 1.0f32 } else { 0.0f32 } },
                            &Default::default()).unwrap();
            }
        }

        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::CursorMoved {position: (x, y), ..}  => {
                        position_x = if x > 1200.0 { 1200.0 - 1.0 } else if x < 0.0 { 0.0 } else { x };
                        position_y = if y > 1200.0 { 1200.0 - 1.0 } else if y < 0.0 { 0.0 } else { y };
                    },
                    glutin::WindowEvent::MouseInput {state, ..}  => {
                        match state {
                            glutin::ElementState::Pressed => {
                                let x = (position_x / 60.0) as u32;
                                let y = (position_y / 60.0) as u32;
                                fields[(y * 20 + x) as usize] = true; 
                            },
                            glutin::ElementState::Released => (),
                        };
                    },
                    _ => ()
                },
                _ => (),
            }
        });
    }
}

fn get_square(x: usize, y: usize) -> [squares::Vertex; 4] {
    use squares::VERTICES;
    [VERTICES[x + y * 21], VERTICES[x + y * 21 + 1], VERTICES[x + (y + 1) * 21], VERTICES[x + (y + 1) * 21 + 1]]
}


