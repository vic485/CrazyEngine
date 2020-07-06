use std::{
    ffi::CString,
    os::raw::c_void,
};

#[macro_use] extern crate log;

pub mod pipeline;
pub mod camera;
pub mod x3d;
pub mod surface;

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "uv", repr = "[f32; 3]", wrapper = "VertexUV")]
    UV,
}

#[derive(Vertex, Copy, Clone)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
    pub normal: VertexNormal,
    pub uv: VertexUV,
    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

pub type VertexIndex = u32;

pub struct BackBuffer {
    window_size: (u32, u32),
}
