use crate::photic_binding::x3d_binding::X3DRenderer;

use std::cell::RefCell;
use std::rc::Rc;

use photic::{
    pipeline::{
        material::{Material, Material2, IsMaterial},
        shader::{Shader, ShaderSource},
        render_mesh::RenderMesh,
    },
    x3d::Renderer3D,
    surface::Surface,

    Vertex,
    VertexIndex,
    VertexPosition,
    VertexRGB,
    VertexNormal,
    VertexUV
};

pub struct X3DMesh {
    pub mesh: Rc<RefCell<RenderMesh>>,
}

impl X3DMesh {
    //TODO: Custom Result needed, this won't work for FFI
    pub fn new(ctx: &mut Surface, triangulated: bool, vertices: Vec<Vertex>, indices: Vec<VertexIndex>) -> Result<Self, String> {
        let mesh = match RenderMesh::new(ctx, triangulated, vertices, indices) {
            Ok(mesh) => mesh,
            Err(err) => {
                error!("Failed to create mesh! {:?}", err);
                return Err(format!("{:?}", err));
            },
        };

        Ok(Self {
            mesh: Rc::new(RefCell::new(mesh)),
        })
    }

    pub fn TRIANGLE(ctx: &mut Surface) -> Result<Self, String> {
        let mesh = match RenderMesh::TRIANGLE(ctx) {
            Ok(mesh) => mesh,
            Err(err) => {
                error!("Failed to create mesh! {:?}", err);
                return Err(format!("{:?}", err));
            },
        };

        Ok(Self {
            mesh: Rc::new(RefCell::new(mesh)),
        })
    }
}

#[derive(Debug)]
struct MeshError {
    err: String,
    code: i32,
}

impl std::fmt::Display for MeshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl std::error::Error for MeshError {}

#[no_mangle]
pub extern "C" fn x3d_new_mesh<'a>(renderer_ptr: *mut X3DRenderer<'a>) -> *mut X3DMesh {
    if renderer_ptr.is_null() {
        crate::ffi::update_last_error(MeshError {
            err: "Pointer to renderer is invalid!".to_string(),
            code: 1,
        }, 1);
        return std::ptr::null_mut();
    }

    let rend = unsafe { &mut *renderer_ptr };

    match X3DMesh::TRIANGLE(&mut rend.surface) {
        Ok(mesh) => Box::into_raw(Box::new(mesh)),
        Err(err) => {
            crate::ffi::update_last_error(MeshError {
                err: err,
                code: 2,
            }, 2);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn x3d_drop_mesh(ptr: *mut X3DMesh) {
    if ptr.is_null() {
        return; //Invalid pointer, scary stuff
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
