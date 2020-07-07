use std::cell::RefCell;
use std::rc::Rc;

use photic::{
    pipeline::{
        material::{Material, Material2, IsMaterial},
        shader::{Shader, ShaderSource},
        render_mesh::RenderMesh,
    },
    x3d::Renderer3D,
    surface::Surface
};

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
pub extern "C" fn x3d_drop_renderer<'a>(ptr: *mut X3DRenderer<'a>) {
    if ptr.is_null() {
        return; //There's nothing to drop, which is scary
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn x3d_renderer_prepare_frame<'a>(ptr: *mut X3DRenderer<'a>) {
    let object = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    object.prepare_frame();
}
