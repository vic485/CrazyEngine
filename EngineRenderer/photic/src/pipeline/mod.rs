pub mod material;
pub mod render_mesh;
pub mod shader;
pub mod texture;
pub mod light;
pub mod light_conversion;

use luminance::{
    shader::program::{
        Uniform,
        Uniformable,
    },
    linear::M44
};
use luminance_derive::UniformInterface;

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)] //Tells luminance that it shouldn't generate an error if the GPU variable doesn't exist
    pub projection: Uniform<M44>,
    #[uniform(unbound)] //#[uniform(name = "foo")] can be used to rename a uniform
    pub view: Uniform<M44>,
    #[uniform(unbound)]
    pub model: Uniform<M44>,
}

//TODO: Add many more types
#[derive(PartialEq)]
pub enum UniformType {
    //Basic data types
    Float,
    Integer,

    //Vectors
    Vector2,
    Vector3,
    Vector4,

    IVector2,
    IVector3,
    IVector4,

    //Matrices
    Matrix2,
    Matrix3,
    Matrix4,

    Matrix2x3,
    Matrix2x4,
    Matrix3x2,
    Matrix3x4,
    Matrix4x2,
    Matrix4x3,

    //Other data types
    Sampler2D,
    Other(String)
}
