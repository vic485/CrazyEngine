use glow::HasContext;
use gl::types::GLuint;

use cgmath::*;

use std::sync::atomic::{AtomicI32, Ordering};

static DIRECTIONAL_LIGHT_COUNT: AtomicI32 = AtomicI32::new(0);

pub trait IsLight {
    fn count_ref(&self) -> &AtomicI32;
    fn upload_fields(&self, gl: &glow::Context, handle: GLuint);
}

pub struct DirectionalLight {
    pub rotation: Quaternion<f32>,

    pub lux: f32,
    pub colour: [f32; 3],
}

impl DirectionalLight {
    pub fn new(rotation: Quaternion<f32>, lux: f32, colour: [f32; 3]) -> Self {
        Self {
            rotation: rotation,

            lux: lux,
            colour: colour,
        }
    }
}

impl IsLight for DirectionalLight {
    fn count_ref(&self) -> &AtomicI32 {
        &DIRECTIONAL_LIGHT_COUNT
    }

    fn upload_fields(&self, gl: &glow::Context, handle: GLuint) {
        let count = DIRECTIONAL_LIGHT_COUNT.load(Ordering::Relaxed);

        unsafe {
            let base = &format!("lights_dir[{}]", count);

            let dir: Vector3<f32> = self.rotation * Vector3::new(1.0, 0.0, 0.0); //TODO: optimise this
            let dir_loc = gl.get_uniform_location(handle, &(base.to_owned() + ".lightDir"));
            gl.uniform_3_f32(dir_loc, dir.x, dir.y, dir.z);

            let lum_loc = gl.get_uniform_location(handle, &(base.to_owned() + ".intensity"));
            gl.uniform_1_f32(lum_loc, self.lux * 125.0);

            let count_loc = gl.get_uniform_location(handle, "lights_dir_count");
            gl.uniform_1_i32(count_loc, count + 1);
        }

        DIRECTIONAL_LIGHT_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}
