use std::cell::RefCell;
use std::rc::Rc;

use photic::{
    pipeline::{
        material::{Material, Material2, IsMaterial},
        shader::{Shader, ShaderSource},
        render_mesh::RenderMesh,
    },
    x3d::Renderer3D
};

pub struct X3D_Renderer<'a> {
    pub surface: photic::surface::Surface,
    pub gl: glow::Context,

    pub renderer: Rc<RefCell<Renderer3D<'a>>>,
}

impl<'a> X3D_Renderer<'a> {
    pub fn new(surface: photic::surface::Surface, gl: glow::Context) -> Self {
        Self {
            surface: surface,
            gl: gl,

            renderer: Rc::new(RefCell::new(Renderer3D::new()))
        }
    }

    pub fn prepare_frame(&self) {
        self.renderer.borrow_mut().prepare_frame(&self.gl);
    }
}

#[no_mangle]
pub extern "C" fn x3d_new_renderer<'a>(size_x: u32, size_y: u32) -> *mut X3D_Renderer<'a> {
    let gl = crate::gl_callback();
    let surface = match photic::surface::Surface::new((size_x, size_y)) {
        Ok(surface) => surface,
        Err(why) => {
            println!("Failed to create rendering surface! Reason: {}", why);
            // return None;
            panic!("Failed to create rendering surface!"); //Technically undefined behaviour, but I honestly do not know how to fix this :(
        }
    };

    let renderer = X3D_Renderer::new(surface, gl);

    Box::into_raw(Box::new(renderer))
}

#[no_mangle]
pub extern "C" fn x3d_drop_renderer<'a>(ptr: *mut X3D_Renderer<'a>) {
    if ptr.is_null() {
        return; //There's nothing to drop, which is scary
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn x3d_renderer_prepare_frame<'a>(ptr: *mut X3D_Renderer<'a>) {
    let object = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    object.prepare_frame();
}
