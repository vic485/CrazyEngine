use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3, Vector4, Quaternion, Rotation3};

use gl::types::GLuint;

use glow::HasContext;

pub trait IsCamera {
    fn get_proj(&self, width: u32, height: u32) -> Matrix4<f32>;
    fn get_view(&self) -> Matrix4<f32>;
    fn upload_fields(&self, gl: &glow::Context, handle: GLuint);
}

pub struct Camera {
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub aperture: f32,
    pub shutter_speed: f32,
    pub iso: f32,

    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Camera {
    pub fn new(fovy: f32, z_near: f32, z_far: f32, aperture: f32, shutter_speed: f32, iso: f32, position: Vector3<f32>, rotation: Quaternion<f32>) -> Camera { //eye: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>
        Camera {
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

    pub fn default() -> Camera {
        Camera {
            fovy: 60.0,
            z_near: 0.02,
            z_far: 100.0,

            aperture: 16.0,
            shutter_speed: 1.0 / 100.0,
            iso: 100.0,

            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::<f32>::from_angle_y(Rad(0.0)), //Rotation3::<f32>::from_angle_y(Rad(3.14 / 2.0))
        }
    }
}

impl IsCamera for Camera {
    fn get_proj(&self, width: u32, height: u32) -> Matrix4<f32> {
        perspective(Rad(self.fovy / 180.0 * std::f32::consts::PI), width as f32 / height as f32, self.z_near, self.z_far)
    }

    fn get_view(&self) -> Matrix4<f32> {
        let rot_view: Matrix4<f32> = self.rotation.into();
        let pos_view: Matrix4<f32> = Matrix4::<f32>::from_translation(-self.position);

        rot_view * pos_view
    }

    fn upload_fields(&self, gl: &glow::Context, handle: GLuint) {
        unsafe {
            let pos_loc = gl.get_uniform_location(handle, "camera.position");
            gl.uniform_3_f32(pos_loc, self.position.x, self.position.y, self.position.z);

            let set_loc = gl.get_uniform_location(handle, "camera.settings");
            gl.uniform_3_f32(set_loc, self.aperture, self.shutter_speed, self.iso);
        }
    }
}
