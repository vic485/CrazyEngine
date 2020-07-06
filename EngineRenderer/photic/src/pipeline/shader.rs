use luminance::shader::program::Program;

use super::{
    ShaderInterface,
    UniformType
};

use std::collections::HashMap;

use regex::Regex;

use crate::VertexSemantics;

pub trait IsShader {
    fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface>;
}

pub struct ShaderSource {
    pub vertex_shader: String,
    pub geometry_shader: Option<String>,
    pub tesselation_shader: Option<String>,
    pub fragment_shader: String,
}

pub struct Shader {
    pub program: Program<VertexSemantics, (), ShaderInterface>,
}

impl Shader {
    pub fn from_source(source: ShaderSource) -> Self {
        let program: Program<VertexSemantics, (), ShaderInterface> = match Program::from_strings(None, &source.vertex_shader, None, &source.fragment_shader) {
            Ok(program) => program.ignore_warnings(),
            Err(err) => {
                error!("{}", err);
                panic!("Failed to compile shaders!");
            }
        };

        Self {
            program: program,
        }
    }
}

impl IsShader for Shader {
    fn program(&self) -> &Program<VertexSemantics, (), ShaderInterface> {
        &self.program
    }
}
