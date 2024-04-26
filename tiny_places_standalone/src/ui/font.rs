use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use graphics::{Viewport, Context, Graphics, ImageSize};


pub struct UiFont {
    face: freetype::Face,
}

impl UiFont {
    pub fn new() -> UiFont {
        let ft = freetype::Library::init().unwrap();
        let font = "resources/font/FiraSans-Regular.ttf";
        let mut face = ft.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, 48).unwrap();
        // let glyphs = glyphs(&mut face, "Hello Piston!");        

        UiFont {
            face,
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
