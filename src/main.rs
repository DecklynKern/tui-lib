#[macro_use]
extern crate glium;
use glium::Surface;

const VERTEX_SHADER: &'static str = "
#version 140

in vec2 position;
out vec2 pos;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    pos = position;
}
";

const FRAGMENT_SHADER: &'static str = "
#version 430

in vec2 pos;
out vec4 color;

uniform uint screenWidth;
uniform uint screenHeight;

uniform CodePage {
    uint code_page[512];
};

bool getCodePagePixel(uint char, uint x, uint y) {
    uint bitIdx = (y % 4) * 8 + x;
    return bool(code_page[char * 2 + y / 4] << (31 - bitIdx) >> 31);
}

void main() {

    uint pixelX = uint((pos.x + 1.0) * float(screenWidth) / 2.0);
    uint pixelY = uint((pos.y + 1.0) * float(screenHeight) / 2.0);

    uint cellX = pixelX / 8;
    uint cellY = pixelY / 8;

    bool pixelValue = getCodePagePixel((cellX + cellY) % 256, pixelX % 8, pixelY % 8);
    color = pixelValue ? vec4(1.0, 1.0, 1.0, 1.0) : vec4(0.0, 0.0, 0.0, 1.0);

}
";

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub fn loadCodePage() -> [u32; 512] {

    let img = image::load(std::io::Cursor::new(&include_bytes!("../codepage.png")), image::ImageFormat::Png).unwrap().to_rgb8();

    let mut data = [0; 512];

    for char_y in 0..16 {
        for char_x in 0..16 {

            for x in 0..8 {
                for y in 0..8 {

                    if img.get_pixel(char_x * 8 + x, char_y * 8 + y).0[0] != 0 {
                        data[((char_x + 16 * char_y) * 2 + y / 4) as usize] |= 1 << ((y % 4) * 8 + x);
                    }
                }
            }
        }
    }

    data

}

fn main() {

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    let cfg: glium::glutin::config::ConfigTemplateBuilder = glium::glutin::config::ConfigTemplateBuilder::new().with_multisampling(1);

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("TUI")
        .with_config_template_builder(cfg)
        .build(&event_loop);

    let shape = vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let draw_params = glium::DrawParameters {
        multisampling: false,
        dithering: false,
        smooth: None,
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        blend: glium::Blend {
            color: glium::BlendingFunction::AlwaysReplace,
            alpha: glium::BlendingFunction::AlwaysReplace,
            constant_value: (1.0, 1.0, 1.0, 1.0)
        },
        .. Default::default()
    };

    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let code_page_data = loadCodePage();
    let code_page_buffer = glium::uniforms::UniformBuffer::new(&display, code_page_data).unwrap();

    let mut screen_width: u32 = 0;
    let mut screen_height: u32 = 0;

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {

        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::Resized(new_size) => {
                    screen_width = new_size.width;
                    screen_height = new_size.height;
                }
                glium::winit::event::WindowEvent::RedrawRequested => {

                    let uniforms = uniform! {
                        screenWidth: screen_width,
                        screenHeight: screen_height,
                        CodePage: &code_page_buffer,
                    };

                    let mut target = display.draw();
                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                        &draw_params).unwrap();
                    target.finish().unwrap();

                }
                _ => (),
            },
            _ => (),
        }
    })
    .unwrap();
}