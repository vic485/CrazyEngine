use crate::photic_binding::x3d_binding::X3DRenderer;

use std::cell::RefCell;
use std::rc::Rc;

use photic::{
    pipeline::{
        material::{Material, Material2, IsMaterial},
        shader::{Shader, ShaderSource},
        render_mesh::RenderMesh,
    },
    camera::Camera,
    x3d::Renderer3D,
    surface::Surface,

    Vertex,
    VertexIndex,
    VertexPosition,
    VertexRGB,
    VertexNormal,
    VertexUV
};

use cgmath::*;

pub struct X3DCamera {
    pub camera: Camera,
}

impl X3DCamera {
    pub fn new(fovy: f32, z_near: f32, z_far: f32, aperture: f32, shutter_speed: f32, iso: f32, position: Vector3<f32>, rotation: Quaternion<f32>) -> Self {
        Self {
            camera: Camera {
                fovy: fovy,
                z_near: z_near,
                z_far: z_far,

                aperture: aperture,
                shutter_speed: shutter_speed,
                iso: iso,

                position: position,
                rotation: rotation,
            }
        }
    }

    pub fn default() -> Self {
        Self {
            camera: Camera::default()
        }
    }
}

#[no_mangle]
pub extern "C" fn x3d_new_camera(fovy: f32, z_near: f32, z_far: f32, aperture: f32, shutter_speed: f32, iso: f32, position: Vector3<f32>) -> *mut X3DCamera { //, rotation: Quaternion<f32>
    let rotation = Rotation3::<f32>::from_angle_y(Rad(0.0));

    Box::into_raw(Box::new(X3DCamera::new(fovy, z_near, z_far, aperture, shutter_speed, iso, position, rotation)))
}

#[no_mangle]
pub extern "C" fn x3d_new_camera_default() -> *mut X3DCamera {
    Box::into_raw(Box::new(X3DCamera::default()))
}

#[no_mangle]
pub extern "C" fn x3d_drop_camera(ptr: *mut X3DCamera) {
    if ptr.is_null() {
        return; //Invalid pointer, scary stuff
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
