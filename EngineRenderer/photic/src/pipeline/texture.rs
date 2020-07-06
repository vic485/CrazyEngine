use glow::HasContext;

use std::convert::TryInto;

pub struct Texture {
    pub gl_texture: <glow::Context as glow::HasContext>::Texture,
}

impl Texture {
    pub fn from_rgba_image(gl: &glow::Context, img: image::RgbaImage) -> Self {
        unsafe {
            let gl_texture = gl.create_texture().expect("Failed to create texture!");
            gl.bind_texture(glow::TEXTURE_2D, Some(gl_texture));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA.try_into().expect("Failed to convert GL_RGB"), img.width().try_into().unwrap(), img.height().try_into().unwrap(), 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&img.into_raw()));
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);

            Self {
                gl_texture: gl_texture
            }
        }
    }
}
