//! This example uses the `glium` library, a high level, safe wrapper for OpenGL.
//!
//! It is based off the example in the [official tutorial].
//!
//! [official tutorial]: http://tomaka.github.io/glium/book/index.html

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

/// Define a struct to store vertices. This struct will be used by `glium` directly.
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex1 = Vertex { position: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.5, 0.0] };
    let vertex3 = Vertex { position: [0.0, 0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Define the shaders.
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 a_color;

        void main() {
            a_color = position;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 a_color;
        out vec4 color;

        void main() {
            color = vec4(a_color, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let draw = || {
        let mut target = display.draw();
        target.clear_color(0.3, 0.3, 0.3, 0.0);
        target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();
    };

    // Finally, draw the triangle!
    draw();

    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(..) => draw(),
                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
