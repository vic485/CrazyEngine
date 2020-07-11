use photic::pipeline::{
    material::{Material2, IsMaterial},
    shader::{Shader, IsShader},
};

use super::shader_binding::X3DShader;

pub struct X3DMaterial<'a> {
    pub material: Material2<'a>,
}

impl<'a> X3DMaterial<'a> {
    pub fn new(shader: &'a dyn IsShader, albedo: [f32; 4], metalness: f32, roughness: f32) -> Self {
        Self {
            material: Material2::new(Box::new(shader), albedo, metalness, roughness),
        }
    }
}

//TODO: Do error checking lol
#[no_mangle]
pub extern "C" fn x3d_new_material<'a>(shader_ptr: *mut X3DShader) -> *mut X3DMaterial<'a> {
    let shader = unsafe {
        assert!(!shader_ptr.is_null());
        &*shader_ptr
    };
    Box::into_raw(Box::new(X3DMaterial::new(&shader.shader, [1.0, 1.0, 1.0, 1.0], 0.0, 0.85)))
}

#[no_mangle]
pub extern "C" fn x3d_drop_material(ptr: *mut X3DMaterial) {
    if ptr.is_null() {
        return; //Invalid pointer, scary stuff
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
