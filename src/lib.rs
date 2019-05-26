extern crate serde;

use serde::{Serialize, Deserialize};

use std::ops::{Add, Sub, Mul};
use std::convert::{From};
use std::f32::consts::PI;

pub fn deg_to_rad(deg: f32) -> f32 { ( deg * PI ) / 180. }

// Vector2 {{{
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Add<Vector2> for Vector2 {
  type Output = Self;

  fn add(self, other: Vector2) -> Self {
    Vector2 { x: self.x + other.x
            , y: self.y + other.y }
  }
}

impl Add<f32> for Vector2 {
  type Output = Self;

  fn add(self, scalar: f32) -> Self {
    Vector2 { x: self.x + scalar
            , y: self.y + scalar }
  }
}

impl Sub<Vector2> for Vector2 {
  type Output = Self;

  fn sub(self, other: Vector2) -> Self {
    Vector2 { x: self.x - other.x
            , y: self.y - other.y }
  }
}

impl Sub<f32> for Vector2 {
  type Output = Self;

  fn sub(self, scalar: f32) -> Self {
    Vector2 { x: self.x - scalar
            , y: self.y - scalar }
  }
}

impl Mul<&Vector2> for &Vector2 {
  type Output = Vector2;

  fn mul(self, other: &Vector2) -> Vector2 {
    Vector2 { x: self.x * other.x
            , y: self.y * other.y }
  }
}

impl Mul<Vector2> for Vector2 {
  type Output = Self;

  fn mul(self, other: Vector2) -> Self {&self * &other}
}

impl Mul<f32> for Vector2 {
  type Output = Vector2;

  fn mul(self, scalar: f32) -> Vector2 {
    Vector2 { x: self.x * scalar
            , y: self.y * scalar }
  }
}

impl Mul<Vector2> for f32 {
  type Output = Vector2;

  fn mul(self, vec: Vector2) -> Vector2 { vec * self }
}

impl From<[f32;2]> for Vector2 {
  fn from(slice: [f32;2]) -> Vector2 {
    Vector2::new(slice[0], slice[1])
  }
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Vector2 {
    Vector2 { x: x, y: y }
  }
  
  pub fn zero() -> Vector2 {
    Vector2::new(0., 0.) 
  }
}
// }}}

// Vector3 {{{
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Add<Vector3> for Vector3 {
  type Output = Self;

  fn add(self, other: Vector3) -> Self {
    Vector3 { x: self.x + other.x
            , y: self.y + other.y
            , z: self.z + other.z }
  }
}

impl Add<f32> for Vector3 {
  type Output = Self;

  fn add(self, scalar: f32) -> Self {
    Vector3 { x: self.x + scalar
            , y: self.y + scalar
            , z: self.z + scalar }
  }
}

impl Sub<Vector3> for Vector3 {
  type Output = Self;

  fn sub(self, other: Vector3) -> Self {
    Vector3 { x: self.x - other.x
            , y: self.y - other.y
            , z: self.z - other.z }
  }
}

impl Sub<f32> for Vector3 {
  type Output = Self;

  fn sub(self, scalar: f32) -> Self {
    Vector3 { x: self.x - scalar
            , y: self.y - scalar
            , z: self.z - scalar }
  }
}

impl Mul<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn mul(self, other: &Vector3) -> Vector3 {
    Vector3 { x: self.x * other.x
            , y: self.y * other.y
            , z: self.z * other.z }
  }
}

impl Mul<Vector3> for Vector3 {
  type Output = Self;

  fn mul(self, other: Vector3) -> Self {&self * &other}
}

impl Mul<f32> for &Vector3 {
  type Output = Vector3;

  fn mul(self, scalar: f32) -> Vector3 {
    Vector3 { x: self.x * scalar
            , y: self.y * scalar
            , z: self.z * scalar }
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;

  fn mul(self, scalar: f32) -> Vector3 { &self * scalar }
}

impl From<[f32;3]> for Vector3 {
  fn from(slice: [f32;3]) -> Vector3 {
    Vector3::new(slice[0], slice[1], slice[2])
  }
}

impl Vector3 {
  pub fn zero() -> Vector3 {
    Vector3 { x: 0., y: 0., z: 0. }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x: x, y: y, z: z }
  }

  pub fn from_polar(r: f32, theta: f32, phi: f32)
    -> Vector3
  {
    Vector3 { x: r * theta.sin() * phi.cos()
            , y: r * theta.sin() * phi.sin()
            , z: r * theta.cos() }
  }

  pub fn to_slice(&self) -> [f32;3] {
    [self.x, self.y, self.z]
  }

  pub fn normalize(&self) -> Vector3 {
    self * (1. / self.magnitude())
  }

  pub fn theta(&self) -> f32 {
    ( self.z / self.magnitude() ).acos()
  }

  pub fn phi(&self) -> f32 {
    self.y.atan2(self.x)
  }

  pub fn magnitude(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
      .sqrt()
  }
  
  pub fn xy(&self) -> Vector2 {
    Vector2::new(self.x, self.y) 
  }
  
  pub fn cross(&self, other: &Vector3) -> Vector3 {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;
    Vector3::new(x, y, z)
  }
}
// }}}

// Vector4 {{{
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

impl Add<Vector4> for Vector4 {
  type Output = Self;

  fn add(self, other: Vector4) -> Self {
    Vector4 { x: self.x + other.x
            , y: self.y + other.y
            , z: self.z + other.z
            , w: self.w + other.w }
  }
}

impl Add<f32> for Vector4 {
  type Output = Self;

  fn add(self, scalar: f32) -> Self {
    Vector4 { x: self.x + scalar
            , y: self.y + scalar
            , z: self.z + scalar
            , w: self.w + scalar }
  }
}

impl Sub<Vector4> for Vector4 {
  type Output = Self;

  fn sub(self, other: Vector4) -> Self {
    Vector4 { x: self.x - other.x
            , y: self.y - other.y
            , z: self.z - other.z
            , w: self.w - other.w }
  }
}

impl Sub<f32> for Vector4 {
  type Output = Self;

  fn sub(self, scalar: f32) -> Self {
    Vector4 { x: self.x - scalar
            , y: self.y - scalar
            , z: self.z - scalar
            , w: self.w - scalar }
  }
}

impl Mul<&Vector4> for &Vector4 {
  type Output = Vector4;

  fn mul(self, other: &Vector4) -> Vector4 {
    Vector4 { x: self.x * other.x
            , y: self.y * other.y
            , z: self.z * other.z
            , w: self.w * other.w }
  }
}

impl Mul<Vector4> for Vector4 {
  type Output = Self;

  fn mul(self, other: Vector4) -> Self {&self * &other}
}

impl Mul<f32> for Vector4 {
  type Output = Vector4;

  fn mul(self, scalar: f32) -> Vector4 {
    Vector4 { x: self.x * scalar
            , y: self.y * scalar
            , z: self.z * scalar
            , w: self.w * scalar }
  }
}

impl From<[f32;4]> for Vector4 {
  fn from(slice: [f32;4]) -> Vector4 {
    Vector4::new(slice[0], slice[1], slice[2], slice[3])
  }
}

impl Vector4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
    Vector4 { x: x, y: y, z: z, w: w }
  }

  pub fn to_slice(&self) -> [f32;4] {
    [self.x, self.y, self.z, self.w]
  }
}
// }}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
