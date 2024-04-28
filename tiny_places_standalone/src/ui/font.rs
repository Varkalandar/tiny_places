use core::cmp::max;

use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use graphics::{Viewport, Context, Graphics, ImageSize};

const PITCH: u32 = 1024;

struct UiGlyph {
    pub metrics: freetype::GlyphMetrics,
    tex_x: u32,
    tex_y: u32,
    advance: u32,
    top: i32, // pixels above the baseline
}

pub struct UiFont {
    face: freetype::Face,
    lineheight: u32,
}

impl UiFont {
    pub fn new(size: u32) -> UiFont {
        let ft = freetype::Library::init().unwrap();
        let font = "resources/font/FiraSans-Regular.ttf";
        let mut face = ft.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, size).unwrap();

        let lineheight = (face.ascender() + face.descender() + 1) as u32; // TODO: line gap?

        create_glyphs(&face,lineheight);

        UiFont {
            face,
            lineheight,
        }        
    }



    /*
    fn render_text<G, T>(&self, c: &Context, gl: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize
    {
        for &(ref texture, [x, y]) in glyphs {
            use graphics::*;
    
            Image::new_color(color::BLACK).draw(
                texture,
                &c.draw_state,
                c.transform.trans(x, y),
                gl
            );
        }
    }
   */

}


fn glyphs(face: &mut freetype::Face, text: &str) -> Vec<(Texture, [f64; 2])> {
    let mut x = 10;
    let mut y = 0;
    let mut res = vec![];
    for ch in text.chars() {
        face.load_char(ch as usize, freetype::face::LoadFlag::RENDER).unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new()
        ).unwrap();
        res.push((texture, [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64]));

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
    res
}




pub fn create_glyphs(face: &freetype::Face, lineheight: u32) -> Texture {
    
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

    let mut buffer = vec![0_u8; (b_width * b_height * 4) as usize];

    // cursor to write glyphs into the texture buffer
    let mut cursor: (u32, u32) = (0, 0);

    for glyph_nr in 0..0xFFFF {

        let idx_result = face.get_char_index(glyph_nr);
        if idx_result.is_ok() {
            let ch = idx_result.unwrap();
            let idx = ch.get() as usize;
            face.load_char(idx, freetype::face::LoadFlag::RENDER).unwrap();
    
            let gs = face.glyph();
            let bitmap = gs.bitmap();
            let m = gs.metrics();
            
            let ascend = face.ascender() as i32 / 64;

            let ug = UiGlyph {
                metrics: m,
                tex_x: cursor.0,
                tex_y: cursor.1,
                advance: (m.horiAdvance / 64) as u32,
                top: gs.bitmap_top(),
            };

            println!("glyph {} has advance={}, ascend={}", idx, ug.advance / 64, ascend);
            
            cursor = convert_bitmap(&mut buffer, &bitmap, &ug, cursor, lineheight);

        }
    }

    Texture::from_memory_alpha(&buffer, b_width, b_height, &TextureSettings::new()).unwrap()
}





    
fn convert_bitmap(buffer: &mut Vec<u8>, bitmap: &freetype::Bitmap, glyph: &UiGlyph, cursor: (u32, u32), lineheight:u32) -> (u32, u32) {


    // now render into cache
    // the bitmap is at slot->bitmap
    // the glyph base is at slot->bitmap_left, CELL_HEIGHT - slot->bitmap_top

    // set glyph size
    /*
    glyph.height  = bitmap.rows;
    glyph.width   = bitmap.width;
    glyph.advance = bitmap.width+1;
    */
    
    // Hajo: the bitmaps are all top aligned. Bitmap top is the ascent
    // above the base line
    // to find the real top position, we must take the font ascent
    // and reduce it by the glyph ascent
    
    let bb = bitmap.buffer();
    let bw = bitmap.width() as u32;
    let bh = bitmap.rows() as u32;
    let bp = bitmap.pitch() as u32;
    
    
    let mut xp = cursor.0 + bw; 
    let mut yp = cursor.1;
    if xp >= PITCH {
         xp = 0;
         yp += lineheight;
    }
    
    println!("placing glyph at {}, {}", xp, yp);
                
    for y in 0..bh {
        for x in 0..bw {
            let idx = (y * bp + x) as usize;
            if bb[idx] > 127 {
                print!("#");
                buffer_setpix(buffer, xp + x, yp + y, bb[idx])                
            } else {
                print!(" ");                
            }
        }
        println!("");                
    }

    (xp, yp)
}


fn buffer_setpix(buffer: &mut Vec<u8>, x: u32, y: u32, alpha: u8) {
    let dpos = ((y * PITCH) + x * 4) as usize;

    buffer[dpos] = 255; // red
    buffer[dpos+1] = 255; // green
    buffer[dpos+2] = 255; // blue
    buffer[dpos+3] = alpha; // alpha
}

/*
void convert_glyph(font_t::glyph_t glyph, uint8_t * texture, int scanwidth)
{
    int gx = (glyph.sheet_index & 31) * 32;
    int gy = (glyph.sheet_index / 32) * 32;

    // dbg->message("convert_glyph()", "sheet pos %d, %d, wh = %d, %d", gx, gy, glyph.height, glyph.width);

    for(int y=0; y<glyph.height; y++) {
        for(int x=0; x<glyph.width; x++) {
            uint8 alpha = glyph.bitmap[y*glyph.width + x];

            texture_setpix(texture, scanwidth, gx+x, gy+y, alpha);
        }
    }
}
*/