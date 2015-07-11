#![feature(plugin)]

#![plugin(simd)]

#[simd]
pub fn transform_add(x: f32, y: f32) -> f32 {
    x + y
}

