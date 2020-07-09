use std::cell::RefCell;
use std::rc::Rc;

use super::mesh_binding::X3DMesh;
use super::camera_binding::X3DCamera;

use photic::{
    pipeline::{
        material::{Material, Material2, IsMaterial},
        shader::{Shader, ShaderSource},
        render_mesh::RenderMesh,
    },
    camera::{Camera, IsCamera},
    x3d::Renderer3D,
    surface::Surface
};

use cgmath::*;

pub struct X3DRenderer<'a> {
    pub surface: Surface,
    pub gl: glow::Context,

    pub renderer: Rc<RefCell<Renderer3D<'a>>>,
}

impl<'a> X3DRenderer<'a> {
    pub fn new(surface: Surface, gl: glow::Context) -> Self {
        Self {
            surface: surface,
            gl: gl,

            renderer: Rc::new(RefCell::new(Renderer3D::new()))
        }
    }

    pub fn prepare_frame(&self) {
        self.renderer.borrow_mut().prepare_frame(&self.gl);
    }

    pub fn finish_frame(&self) {
        self.renderer.borrow_mut().finish_frame();
    }

    pub fn draw_mesh<T: IsMaterial>(&mut self, camera: &X3DCamera, mesh: &X3DMesh, material: T, model_matrix: Matrix4<f32>) {
        self.renderer.borrow_mut().draw_mesh(&mut self.surface, &self.gl, camera.camera, &mesh.mesh.borrow(), material, model_matrix);
    }
}

#[no_mangle]
pub extern "C" fn x3d_new_renderer<'a>(size_x: u32, size_y: u32) -> *mut X3DRenderer<'a> {
    let gl = crate::gl_callback();
    let surface = match Surface::new((size_x, size_y)) {
        Ok(surface) => surface,
        Err(why) => {
            println!("Failed to create rendering surface! Reason: {}", why);
            // return None;
            panic!("Failed to create rendering surface!"); //Technically undefined behaviour, but I honestly do not know how to fix this :(
        }
    };

    let renderer = X3DRenderer::new(surface, gl);

    Box::into_raw(Box::new(renderer))
}

#[no_mangle]
pub extern "C" fn x3d_drop_renderer(ptr: *mut X3DRenderer) {
    if ptr.is_null() {
        return; //There's nothing to drop, which is scary
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn x3d_renderer_prepare_frame(ptr: *mut X3DRenderer) {
    let object = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    object.prepare_frame();
}

#[no_mangle]
pub extern "C" fn x3d_renderer_finish_frame(ptr: *mut X3DRenderer) {
    let object = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    object.finish_frame();
}

#[no_mangle]
pub extern "C" fn x3d_renderer_draw_mesh(rend_ptr: *mut X3DRenderer, mesh_ptr: *mut X3DMesh, cam_ptr: *mut X3DCamera) {
    let mesh = unsafe {
        assert!(!mesh_ptr.is_null());
        &mut *mesh_ptr
    };
    let renderer = unsafe {
        assert!(!rend_ptr.is_null());
        &mut *rend_ptr
    };

    // let camera = Camera::default();
    let camera = unsafe {
        assert!(!cam_ptr.is_null());
        &mut *cam_ptr
    };

    let shader_source = ShaderSource {
        vertex_shader: include_str!("../../../shaders/vertex.glsl").to_string(),
        geometry_shader: None,
        tesselation_shader: None,
        fragment_shader: include_str!("../../../shaders/fragment.glsl").to_string(),
    };
    let shader = Shader::from_source(shader_source);
    let material = Material2::new(&shader, [1.0, 1.0, 1.0, 1.0], 0.0, 0.85);

    let model_matrix = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, -1.0));

    renderer.draw_mesh(&camera, &mesh, material, model_matrix);
}
