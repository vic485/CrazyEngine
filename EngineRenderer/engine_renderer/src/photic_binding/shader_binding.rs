use std::os::raw::c_char;
use std::ffi::CStr;

use photic::pipeline::shader::{Shader, ShaderSource};

pub struct X3DShader {
    pub shader: Shader,
}

impl X3DShader {
    pub fn new(vertex_shader: String, geometry_shader: Option<String>, tesselation_shader: Option<String>, fragment_shader: String) -> Self {
        let shader_source = ShaderSource {
            vertex_shader: vertex_shader,
            geometry_shader: geometry_shader,
            tesselation_shader: tesselation_shader,
            fragment_shader: fragment_shader,
        };

        Self {
            shader: Shader::from_source(shader_source),
        }
    }
}

//TODO: Do error checking lol
#[no_mangle]
pub extern "C" fn x3d_new_shader(vs: *const c_char, gs: *const c_char, ts: *const c_char, fs: *const c_char) -> *mut X3DShader {
    let vs_c = unsafe {
        assert!(!vs.is_null());
        CStr::from_ptr(vs).to_str().unwrap().to_string()
    };
    let gs_c = unsafe {
        if gs.is_null() { None } else { Some(CStr::from_ptr(gs).to_str().unwrap().to_string()) }
    };
    let ts_c = unsafe {
        if ts.is_null() { None } else { Some(CStr::from_ptr(ts).to_str().unwrap().to_string()) }
    };
    let fs_c = unsafe {
        assert!(!fs.is_null());
        CStr::from_ptr(fs).to_str().unwrap().to_string()
    };
    Box::into_raw(Box::new(X3DShader::new(vs_c, gs_c, ts_c, fs_c)))
}

#[no_mangle]
pub extern "C" fn x3d_drop_shader(ptr: *mut X3DShader) {
    if ptr.is_null() {
        return; //Invalid pointer, scary stuff
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
