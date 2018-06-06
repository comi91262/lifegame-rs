#[macro_use]
extern crate glium;

mod squares;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        .with_title("lifegame");

    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

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

    let mut t: u32 = 1;
    let mut position_x = 0.0;
    let mut position_y = 0.0;

    let mut fields = [false; 512];
    let indices =
        glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1u16, 3u16, 0u16, 2u16, 3u16]).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for i in 0..400 {
            let position = glium::VertexBuffer::new(&display, &[squares::VERTICES[i], squares::VERTICES[i+1], squares::VERTICES[i+21], squares::VERTICES[i+22]]).unwrap();
            target.draw(&position, &indices, &program, &uniform!{t: if fields[i] { 1.0f32 } else { 0.0f32 } }, &Default::default()).unwrap();
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
                                println!("{:?}", x);
                                println!("{:?}", y);
                                println!("{:?}", state);
                                println!("{:?}", fields[(y * 20 + x) as usize]);
                                fields[(y * 20 + x) as usize] = true; 
                                
                            }
                            glutin::ElementState::Released => println!("{:?}", state), 
                        };
                    },
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
