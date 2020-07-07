use crate::{
    pipeline::{
        render_mesh::RenderMesh,
        light::IsLight,
        material::IsMaterial,
    },
    camera::{
        IsCamera,
    }
};

use glow::HasContext;

use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
    tess::TessSliceIndex,

    face_culling::{
        FaceCulling,
        FaceCullingOrder,
        FaceCullingMode
    }
};

use cgmath::*;

use std::ops::Deref;
use std::sync::atomic::Ordering;

pub struct Renderer3D<'a> {
    pub mesh_count: usize,
    pub vert_count: usize,

    pub lights: Vec<&'a dyn IsLight>,
    pub light_count: usize,

    pub render_state: RenderState,
}

impl<'a> Renderer3D<'a> {
    pub fn new() -> Self {
        Self {
            mesh_count: 0,
            vert_count: 0,

            lights: Vec::new(),
            light_count: 0,

            render_state: RenderState::default().set_face_culling(FaceCulling::new(FaceCullingOrder::CCW, FaceCullingMode::Back)),
        }
    }

    pub fn prepare_frame(&mut self, gl: &glow::Context) {
        unsafe {
            gl.clear(glow::DEPTH_BUFFER_BIT);
        }
        self.vert_count = 0;
    }

    pub fn finish_frame(&mut self) {
        //TODO: Optimise this, no need to loop through every light to reset lol
        for light in &self.lights {
            light.count_ref().store(0, Ordering::Relaxed);
        }

        self.light_count = self.lights.len();
        self.lights = Vec::new();
    }

    //Function really only used internally, as it's quite unintuitive lol
    pub fn use_light(&mut self, light: &'a dyn IsLight) {
        self.lights.push(light);
    }

    pub fn draw_mesh(&mut self, surface: &mut crate::surface::Surface, gl: &glow::Context, camera: &dyn IsCamera, mesh: &'a RenderMesh, material: Box<dyn IsMaterial + 'a>, model_matrix: Matrix4<f32>) {
        let back_buffer = surface.back_buffer().expect("Couldn't get the backbuffer!");

        let projection = camera.get_proj(surface.width(), surface.height());
        let view = camera.get_view();

        surface.pipeline_builder().pipeline(
            &back_buffer,
            &PipelineState::default(),
            |_, mut shd_gate| {
                material.bind_texture(gl);
                shd_gate.shade(&material.program(), |iface, mut rdr_gate| {
                    let handle = material.program().deref().handle();

                    for light in &self.lights {
                        light.upload_fields(gl, handle);
                    }

                    camera.upload_fields(gl, handle);
                    iface.projection.update(projection.into());
                    iface.view.update(view.into());
                    material.upload_fields(gl);

                    rdr_gate.render(&self.render_state, |mut tess_gate| {
                        iface.model.update(model_matrix.into()); //tc = transform component
                        tess_gate.render(mesh.tess.slice(..))
                    })
                });
                unsafe { gl.bind_texture(glow::TEXTURE_2D, None); }
            }
        );

        self.vert_count += mesh.vert_count;
    }
}
