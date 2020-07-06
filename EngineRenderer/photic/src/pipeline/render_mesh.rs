use luminance::context::GraphicsContext;
use luminance::tess::{Mode, TessBuilder, Tess, TessError};
use crate::{
    Vertex,
    VertexIndex,
    VertexPosition,
    VertexRGB,
    VertexNormal,
    VertexUV,

    pipeline::material::IsMaterial,
};

pub struct RenderMesh {
    pub tess: Tess,
    pub triangulated: bool,

    pub vert_count: usize,
}

impl RenderMesh {
    pub fn new<C>(ctx: &mut C, triangulated: bool, vertices: Vec<Vertex>, indices: Vec<VertexIndex>) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let vert_count = vertices.len();
        println!("Vertices: {}", vert_count);
        let tess = TessBuilder::new(ctx)
            .set_mode(Mode::Triangle)
            .add_vertices(vertices)
            .set_indices(indices)
            .build()?;
        Ok(RenderMesh {
            tess: tess,
            triangulated: triangulated,

            vert_count: vert_count,
        })
    }

    pub fn from_tess(tess: Tess, triangulated: bool, vert_count: usize) -> RenderMesh {
        RenderMesh {
            tess: tess,
            triangulated: triangulated,
            vert_count: vert_count,
        }
    }

    pub fn TRIANGLE<C>(ctx: &mut C) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let tess = TessBuilder::new(ctx)
            .set_mode(Mode::Triangle)
            .add_vertices(TRIANGLE_VERTICES)
            .build()?;
        Ok(RenderMesh {
            tess: tess,
            triangulated: true,

            vert_count: 3,
        })
    }
}

const TRIANGLE_VERTICES: [Vertex; 3] = [
    Vertex {
        position: VertexPosition::new([-0.5, -0.5, 0.0]),
        color: VertexRGB::new([255, 0, 0]),
        normal: VertexNormal::new([0.0, 0.0, -1.0]),
        uv: VertexUV::new([0.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0.5, -0.5, 0.0]),
        color: VertexRGB::new([0, 255, 0]),
        normal: VertexNormal::new([0.0, 0.0, -1.0]),
        uv: VertexUV::new([1.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0., 0.5, 0.0]),
        color: VertexRGB::new([0, 0, 255]),
        normal: VertexNormal::new([0.0, 0.0, -1.0]),
        uv: VertexUV::new([0.5, 1.0, 0.0]),
    },
];
