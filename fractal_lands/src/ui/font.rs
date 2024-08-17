use std::collections::HashMap;

use sdl2::render::Texture;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureAccess;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;

use crate::texture_from_data;

const PITCH: u32 = 1024;


struct UiGlyph {
    pub metrics: freetype::GlyphMetrics,
    tex_x: u32,
    tex_y: u32,
    advance: f64,
    top: f64, // pixels above the baseline
    left: f64, // left-right shift
    bm_w: f64,
    bm_h: f64,
}


pub struct UiFont {
    face: freetype::Face,
    pub lineheight: i32,
    
    glyphs: HashMap<usize, UiGlyph>,
    texture: Texture,
}


impl UiFont {

    pub fn new<T>(creator: &TextureCreator<T>, size: u32) -> UiFont {
        let ft = freetype::Library::init().unwrap();
        let font = "resources/font/FiraSans-Regular.ttf";
        let face = ft.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, size).unwrap();

        let lineheight = ((face.ascender() - face.descender()) / 64) as i32 + 5; // TODO: line gap?

        // println!("Ascend {} descend {}", face.ascender(), face.descender());

        let mut glyphs = HashMap::new();
        let texture = create_glyphs(creator, &face, &mut glyphs, lineheight as u32);

        UiFont {
            face,
            lineheight,
            glyphs,
            texture,
        }        
    }


    pub fn calc_string_width(&self, text: &str) -> f64
    {
        let mut w = 0.0;
        
        for ch in text.chars() {
            let idx = ch as usize;
            let glyph = self.glyphs.get(&idx).unwrap();
            w += glyph.advance;                
        }

        w
    }


    pub fn draw(&self, x: i32, y: i32, text: &str, color: &[f32; 4])
    {
/*
        gl.draw(viewport, |c, gl| {

            let mut xp = x as f64;
            let yp = (y as f64) + (self.face.ascender() / 64) as f64;
            
            for ch in text.chars() {
                
                // println!("char {} usize {}", ch, ch);
                let idx = ch as usize;
                
                let glyph = self.glyphs.get(&idx).unwrap();

                let image = 
                    Image
                    ::new_color(*color)
                    .src_rect([glyph.tex_x as f64, glyph.tex_y as f64, glyph.bm_w, glyph.bm_h]);
                                    
                image.draw(
                    &self.texture,
                    ds,
                    c.transform.trans(xp + glyph.left, yp - glyph.top),
                    gl
                );
                
                xp += glyph.advance;                
            }
        });
        */
    }
}


fn create_glyphs<T>(creator: &TextureCreator<T>, face: &freetype::Face, glyphs: &mut HashMap<usize, UiGlyph>, lineheight: u32) -> Texture {
    
    let mut num_glyphs = 0;

    for glyph_nr in 0..0xFFFF {
        let idx_result = face.get_char_index(glyph_nr);
        if idx_result.is_ok() { 
            num_glyphs += 1;
        }
    }

    println!("Found {} glyphs in font, lineheight={}", num_glyphs, lineheight);

    let b_width = PITCH;
    let b_height = (num_glyphs / 32) * lineheight;

    let mut buffer = vec![0_u8; (b_width * b_height) as usize];

    // cursor to write glyphs into the texture buffer
    let mut cursor: (u32, u32) = (0, 0);

    for glyph_nr in 0..0xFFFF {

        let idx_result = face.get_char_index(glyph_nr);
        if idx_result.is_ok() {
            let ch = idx_result.unwrap();
            let idx = ch.get();
            face.load_glyph(idx, freetype::face::LoadFlag::RENDER).unwrap();
    
            let gs = face.glyph();
            let bitmap = gs.bitmap();
            let m = gs.metrics();
            
            let ascend = face.ascender() as i32 / 64;

            let ug = UiGlyph {
                metrics: m,
                tex_x: cursor.0,
                tex_y: cursor.1,
                advance: m.horiAdvance as f64 / 64.0,
                top: gs.bitmap_top() as f64,
                left: gs.bitmap_left() as f64,
                bm_w: bitmap.width() as f64,
                bm_h: bitmap.rows() as f64,
            };

            let left = gs.bitmap_left();
            // println!("glyph {} has advance={}, ascend={}, left={}", idx, ug.advance / 64.0, ascend, left);
            
            cursor = convert_bitmap(&mut buffer, &bitmap, cursor, lineheight);

            glyphs.insert(glyph_nr, ug);
        }
    }

    let mut tex =
        creator 
        .create_texture(PixelFormatEnum::RGBA8888, 
        TextureAccess::Static, 
        32, 32).unwrap();
    

    tex
    
    // texture_from_data(&mut canvas.texture_creator(), &buffer, b_width, b_height)
}

    
fn convert_bitmap(buffer: &mut Vec<u8>, bitmap: &freetype::Bitmap,cursor: (u32, u32), lineheight:u32) -> (u32, u32) {
    
    let bb = bitmap.buffer();
    let bw = bitmap.width() as u32;
    let bh = bitmap.rows() as u32;
    let bp = bitmap.pitch() as u32;
    
    let mut xp = cursor.0; 
    let mut yp = cursor.1;
    
    // println!("placing glyph at {}, {}", xp, yp);

    for y in 0..bh {
        for x in 0..bw {
            let idx = (y * bp + x) as usize;
            let alpha = (bb[idx] as f64 / 255.0).powf(0.75) * 255.0;
            buffer_setpix(buffer, xp + x, yp + y, alpha as u8)                
        }
    }

    // debug, print glyph on stdout
    /*
    for y in 0..bh {
        for x in 0..bw {
            let idx = (y * bp + x) as usize;
            if bb[idx] > 127 {
                print!("#");
            } else {
                print!(" ");                
            }
        }
        println!("");                
    }
    */
    
    xp += bw + 1;
    if xp >= PITCH {
         xp = 0;
         yp += lineheight;
    }

    (xp, yp)
}


fn buffer_setpix(buffer: &mut Vec<u8>, x: u32, y: u32, alpha: u8) {
    let dpos = ((y * PITCH) + x) as usize;
    buffer[dpos] = alpha;
}
