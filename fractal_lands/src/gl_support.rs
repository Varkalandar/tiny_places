use std::io::BufReader;
use std::fs::File;

use glutin::surface::SurfaceTypeTrait;
use glutin::surface::ResizeableSurface;
use glutin::surface::WindowSurface;

use glium::Display;
use glium::Texture2d;
use glium::Program;
use glium::Surface;
use glium::Frame;
use glium::implement_vertex;
use glium::uniform;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);


// until there is real support, this can be used
#[derive(Debug)]
pub enum BlendMode {
    Blend,
    Add,
}


pub fn load_texture<T: SurfaceTypeTrait + ResizeableSurface>(display: &Display<T>, filename: &str) -> glium::Texture2d {
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let image = image::load(reader, image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(display, image).unwrap();
    
    texture
}

pub fn texture_from_data<T: SurfaceTypeTrait + ResizeableSurface>(display: &Display<T>, data: &[u8], width: u32, height: u32) -> glium::Texture2d {

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(data, (width, height));
    let texture = glium::Texture2d::new(display, image).unwrap();
    
    texture
}

pub fn build_program(display: &Display<WindowSurface>) -> glium::Program {
    let vertex_shader_src = r#"
    #version 140
    
    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;
    
    uniform mat4 matrix;
    
    void main() {
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;
    
    let fragment_shader_src = r#"
    #version 140
    
    in vec2 v_tex_coords;
    out vec4 color;
    
    uniform sampler2D tex;
    
    void main() {
        color = texture(tex, v_tex_coords);
    }
    "#;
    
    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    program
}

pub fn draw_texture<T: SurfaceTypeTrait + ResizeableSurface>(display: &Display<T>,
                                                             target: &mut Frame,   
                                                             program: &Program,  
                                                             texture: &Texture2d,
                                                             xp: f32,
                                                             yp: f32, 
                                                             sx: f32, 
                                                             sy: f32) {

    let fw = texture.width() as f32 * sx;
    let fh = texture.height() as f32 * sy;

    let shape = vec![
        Vertex { position: [xp + 0.0, yp + 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [xp +  fw, yp + 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [xp +  fw, yp +  fh], tex_coords: [1.0, 1.0] },

        Vertex { position: [xp +  fw,  yp + fh], tex_coords: [1.0, 1.0] },
        Vertex { position: [xp + 0.0,  yp + fh], tex_coords: [0.0, 1.0] },
        Vertex { position: [xp + 0.0, yp + 0.0], tex_coords: [0.0, 0.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let (d_width, d_height) = display.get_framebuffer_dimensions();
    let fdw = d_width as f32;
    let fdh = d_height as f32;

    let xf: f32 = 2.0 / fdw; 
    let yf: f32 = 2.0 / fdh; 

    let uniforms = uniform! {
        matrix: [
            [  xf,  0.0,  0.0,  0.0],
            [ 0.0,   yf,  0.0,  0.0],
            [ 0.0,  0.0,  1.0,  0.0],
            [-1.0, -1.0,  0.0,  1.0],
        ],                        
        tex: texture,                        
    };

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    target.draw(&vertex_buffer, &indices, program, &uniforms, &params).unwrap();
}