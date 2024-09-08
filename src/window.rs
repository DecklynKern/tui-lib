use super::surface::*;
use super::event::*;

use std::time::*;
use glium::Surface;

const CELL_WIDTH: u32 = 8;
const CELL_HEIGHT: u32 = 8;

const VERTEX_SHADER: &str = include_str!("shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("shaders/frag.glsl");

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub fn load_code_page() -> [u32; 512] {

    let img = image::load(std::io::Cursor::new(&include_bytes!("codepage.png")), image::ImageFormat::Png).unwrap().to_rgb8();

    let mut data = [0; 512];

    for char_y in 0..16 {
        for char_x in 0..16 {

            for x in 0..CELL_WIDTH {
                for y in 0..CELL_HEIGHT {

                    if img.get_pixel(char_x * CELL_WIDTH + x, char_y * CELL_HEIGHT + y).0[0] != 0 {
                        data[((char_x + 16 * char_y) * 2 + y / 4) as usize] |= 1 << ((y % 4) * 8 + x);
                    }
                }
            }
        }
    }

    data

}

pub trait State {
    fn handle_event(&mut self, event: Event) {}
    fn tick(&mut self, context: FrameContext) {}
    fn draw(&mut self, context: FrameContext, surface: &mut ScreenSurface) {}
    fn close(&mut self) {}
}

pub struct WindowHandler<S: State> {
    state: S,
    screen_width: u32,
    screen_height: u32,
    screen_cells_width: u32,
    screen_cells_height: u32,
    surface: ScreenSurface,
    target_fps: u64
}

impl<S: State> WindowHandler<S> {

    pub fn new(state: S) -> Self {
        Self {
            state,
            screen_width: 0,
            screen_height: 0,
            screen_cells_width: 0,
            screen_cells_height: 0,
            surface: ScreenSurface::new(0, 0),
            target_fps: 20
        }
    }

    pub fn with_init_screen_size(mut self, screen_width: u32, screen_height: u32) -> Self {

        self.screen_width = screen_width;
        self.screen_height = screen_height;

        self.screen_cells_width = screen_width.div_ceil(CELL_WIDTH);
        self.screen_cells_height = screen_height.div_ceil(CELL_HEIGHT);
        
        self.surface = ScreenSurface::new(self.screen_cells_width as usize, self.screen_cells_height as usize);

        self

    }

    pub fn with_target_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps as u64;
        self
    }

    pub fn run(&mut self) {

        let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();

        let cfg: glium::glutin::config::ConfigTemplateBuilder = glium::glutin::config::ConfigTemplateBuilder::new().with_multisampling(1);
        let mut builder = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("TUI")
            .with_config_template_builder(cfg);

        if self.screen_width != 0 && self.screen_height != 0 {
            builder = builder.with_inner_size(self.screen_width, self.screen_height);
        }

        let (window, display) = builder.build(&event_loop);

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

        let code_page_data = load_code_page();
        let code_page_texture = glium::texture::UnsignedTexture1d::new(&display, code_page_data.as_slice()).unwrap();

        let cells_sample_behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };
        
        let (cells_chars, cells_fgs, cells_bgs) = self.surface.get_raw_slices();
        
        let mut chars_texture = glium::texture::UnsignedTexture1d::new(&display, cells_chars).unwrap();
        let mut fgs_texture = glium::texture::UnsignedTexture1d::new(&display, cells_fgs).unwrap();
        let mut bgs_texture = glium::texture::UnsignedTexture1d::new(&display, cells_bgs).unwrap();

        let mut last_redraw = Instant::now();
        let mut next_redraw = Instant::now();

        let mut last_mouse_position = MousePosition::new();
        let mut mouse_position = MousePosition::new();

        #[allow(deprecated)]
        event_loop.run(move |ev, window_target| {

            if next_redraw <= Instant::now() {
                next_redraw = Instant::now() + Duration::from_millis(1000 / self.target_fps);
                window.request_redraw();
            }

            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    glium::winit::event::WindowEvent::CloseRequested => {
                        self.state.close();
                        window_target.exit();
                    }
                    glium::winit::event::WindowEvent::CursorMoved { position, .. } => {

                        mouse_position.pixel_x = position.x;
                        mouse_position.pixel_y = position.y;

                        mouse_position.pixel_x_rel = mouse_position.pixel_x - last_mouse_position.pixel_x;
                        mouse_position.pixel_y_rel = mouse_position.pixel_y - last_mouse_position.pixel_y;

                        mouse_position.cell_x = (position.x / CELL_WIDTH as f64) as i32;
                        mouse_position.cell_y = (position.y / CELL_HEIGHT as f64) as i32;

                        mouse_position.cell_x_rel = mouse_position.cell_x - last_mouse_position.cell_x;
                        mouse_position.cell_y_rel = mouse_position.cell_y - last_mouse_position.cell_y;

                    }
                    glium::winit::event::WindowEvent::MouseInput { button: winit_button, state, .. } => {

                        let button = MouseButton::from_winit(winit_button);

                        let event = match state {
                            glium::winit::event::ElementState::Pressed => Event::MouseDown(button, mouse_position.clone()),
                            glium::winit::event::ElementState::Released => Event::MouseUp(button)
                        };

                        self.state.handle_event(event);

                    }
                    glium::winit::event::WindowEvent::KeyboardInput {event: key_event, ..} => {

                        let text = key_event.text; // use at some point without making it cringe to use

                        let key = match key_event.physical_key {
                            glium::winit::keyboard::PhysicalKey::Code(key_code) => Key::from_key_code(key_code),
                            glium::winit::keyboard::PhysicalKey::Unidentified(_) => Key::Unknown
                        };

                        let event = match key_event.state {
                            glium::winit::event::ElementState::Pressed => if key_event.repeat {
                                Event::KeyRepeat(key)
                            }
                            else {
                                Event::KeyDown(key)
                            },
                            glium::winit::event::ElementState::Released => Event::KeyUp(key)
                        };

                        self.state.handle_event(event);

                    }
                    glium::winit::event::WindowEvent::Resized(new_size) => {

                        self.screen_width = new_size.width;
                        self.screen_height = new_size.height;

                        self.screen_cells_width = self.screen_width.div_ceil(CELL_WIDTH);
                        self.screen_cells_height = self.screen_height.div_ceil(CELL_HEIGHT);

                        self.surface = ScreenSurface::new(self.screen_cells_width as usize, self.screen_cells_height as usize);

                    }
                    glium::winit::event::WindowEvent::RedrawRequested => {

                        window_target.set_control_flow(glium::winit::event_loop::ControlFlow::wait_duration(Duration::from_millis(1000 / self.target_fps)));

                        let frame_context = FrameContext {
                            dt_seconds: (Instant::now() - last_redraw).as_secs_f32(),
                            mouse_pos: mouse_position.clone()
                        };

                        last_redraw = Instant::now();
                        last_mouse_position = mouse_position.clone();

                        self.state.tick(frame_context.clone());
                        self.state.draw(frame_context, &mut self.surface);
        
                        let (cells_chars, cells_fgs, cells_bgs) = self.surface.get_raw_slices();
                        chars_texture = glium::texture::UnsignedTexture1d::new(&display, cells_chars).unwrap();
                        fgs_texture = glium::texture::UnsignedTexture1d::new(&display, cells_fgs).unwrap();
                        bgs_texture = glium::texture::UnsignedTexture1d::new(&display, cells_bgs).unwrap();

                        let uniforms = uniform! {
                            screenWidth: self.screen_width,
                            screenHeight: self.screen_height,
                            screenCellsWidth: self.screen_cells_width,
                            screenCellsHeight: self.screen_cells_height,
                            code_page: &code_page_texture,
                            chars: glium::uniforms::Sampler(&chars_texture, cells_sample_behavior),
                            fgs: glium::uniforms::Sampler(&fgs_texture, cells_sample_behavior),
                            bgs: glium::uniforms::Sampler(&bgs_texture, cells_sample_behavior)
                        };

                        let mut target = display.draw();
                        target.draw(&vertex_buffer, indices, &program, &uniforms,
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
}