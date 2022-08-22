use crate::{Vector3, Vector4};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

pub trait Matrix:
    Add
    + AddAssign
    + Mul
    + Mul<Vector4, Output = Vector4>
    + MulAssign
    + Sub
    + SubAssign
    + From<[f32; 4 * 4]>
    + Into<[f32; 4 * 4]>
    + Index<(usize, usize)>
    + Display
    + Clone
    + Copy
    + Debug
    + Sized
{
    fn zero() -> Self;
    fn identity() -> Self;

    fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Self;

    fn scale(x: f32, y: f32, z: f32) -> Self;
    fn translation(x: f32, y: f32, z: f32) -> Self;
    fn rotation(x: f32, y: f32, z: f32) -> Self;

    fn rotation_x(angle: f32) -> Self;
    fn rotation_y(angle: f32) -> Self;
    fn rotation_z(angle: f32) -> Self;

    fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Self;
    fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self;

    fn get(&self, col: usize, row: usize) -> f32;
    fn set(&mut self, col: usize, row: usize, value: f32);
}
