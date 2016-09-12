#![crate_type = "lib"]
#![feature(fundamental, no_core, lang_items, custom_attribute, attr_literals)]
#![allow(dead_code)]
#![no_core]

mod core;

#[inspirv(vector(base = "f32", components = 4))]
struct float4;
impl core::marker::Copy for float4 {}

impl float4 {
    #[inspirv(intrinsic = "float4_new")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> float4 {
        float4::new(x, y, z, w)
    }
}

#[inspirv(interface)]
struct VertexInput {
    #[inspirv(location = 0)]
    pos: float4,
}

#[inspirv(interface)]
struct VertexVarying {
    #[inspirv(location = 0)]
    #[inspirv(builtin = "Position")]
    pos: float4,
}

#[inspirv(entry_point = "vertex")]
fn vertex(input: VertexInput) -> VertexVarying {
    float4::new(0.0f32, 1.0f32, 0.0f32, 1.0f32);
    VertexVarying {
        pos: float4::new(1.0f32, 1.0f32, 0.0f32, 1.0f32),
    }
}
