use glow::HasContext;
use std::convert::TryInto;
use std::ops::Deref;

use luminance::shader::program::Program;

use crate::{
    VertexSemantics,
    pipeline::{
        ShaderInterface,
        shader::IsShader,
        texture::Texture,
    },
};

pub trait IsMaterial {
    fn upload_fields(&self, gl: &glow::Context);
    fn bind_texture(&self, gl: &glow::Context);
    fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface>;
}

#[derive(Copy, Clone)]
pub struct Material<'a> {
    pub shader: &'a dyn IsShader,

    pub albedo: [f32; 4],
    pub metalness: f32,
    pub roughness: f32,

    pub main_texture: &'a Texture,
}

impl<'a> Material<'a> {
    pub fn new(shader: &'a dyn IsShader, texture: &'a Texture, albedo: [f32; 4], metalness: f32, roughness: f32) -> Self {
        Self {
            shader: shader,

            albedo: albedo,
            metalness: metalness,
            roughness: roughness,

            main_texture: texture
        }
    }
}

impl<'a> IsMaterial for Material<'a> {
    fn upload_fields(&self, gl: &glow::Context) {
        let handle = self.shader.program().deref().handle();

        unsafe {
            let albedo_loc = gl.get_uniform_location(handle, "material.albedo");
            gl.uniform_4_f32(albedo_loc, self.albedo[0], self.albedo[1], self.albedo[2], self.albedo[3]);

            let metalness_loc = gl.get_uniform_location(handle, "material.metalness");
            gl.uniform_1_f32(metalness_loc, self.metalness);

            let roughness_loc = gl.get_uniform_location(handle, "material.roughness");
            gl.uniform_1_f32(roughness_loc, self.roughness);
        }
    }

    fn bind_texture(&self, gl: &glow::Context) {
        unsafe { gl.bind_texture(glow::TEXTURE_2D, Some(self.main_texture.gl_texture)); }
    }

    fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface> {
        &self.shader.program()
    }
}

#[derive(Copy, Clone)]
pub struct Material2<'a> {
    pub shader: &'a dyn IsShader,

    pub albedo: [f32; 4],
    pub metalness: f32,
    pub roughness: f32,
}

impl<'a> Material2<'a> {
    pub fn new(shader: &'a dyn IsShader, albedo: [f32; 4], metalness: f32, roughness: f32) -> Self {
        Self {
            shader: shader,

            albedo: albedo,
            metalness: metalness,
            roughness: roughness,
        }
    }
}

impl<'a> IsMaterial for Material2<'a> {
    fn upload_fields(&self, gl: &glow::Context) {
        let handle = self.shader.program().deref().handle();

        unsafe {
            let albedo_loc = gl.get_uniform_location(handle, "material.albedo");
            gl.uniform_4_f32(albedo_loc, self.albedo[0], self.albedo[1], self.albedo[2], self.albedo[3]);

            let metalness_loc = gl.get_uniform_location(handle, "material.metalness");
            gl.uniform_1_f32(metalness_loc, self.metalness);

            let roughness_loc = gl.get_uniform_location(handle, "material.roughness");
            gl.uniform_1_f32(roughness_loc, self.roughness);
        }
    }

    fn bind_texture(&self, gl: &glow::Context) {
        // unsafe { gl.bind_texture(glow::TEXTURE_2D, Some(self.main_texture.gl_texture)); }
    }

    fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface> {
        &self.shader.program()
    }
}
