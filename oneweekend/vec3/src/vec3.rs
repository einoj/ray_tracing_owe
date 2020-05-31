use std::ops::Index;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vec3 {
  pub e: [f64; 3],
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        &self.e[index]
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
  type Output = Vec3;

  fn add(self, other: &'b Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] + other.e[0],  self.e[1] + other.e[1], self.e[2] + other.e[2]]}
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 { e: [self.e[0] + other.e[0],  self.e[1] + other.e[1], self.e[2] + other.e[2]]}
  }
}

// Implements dot product 
// WARN Moves Vec3!
impl Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 { e:[self.e[0] * other.e[0],self.e[1] * other.e[1],self.e[2] * other.e[2]]}
  }
}

// Implements scalar product
// needs to be vector * scalar
impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, f: f64) -> Vec3 {
    Vec3 { e:[self.e[0] * f, self.e[1] * f, self.e[2] * f]}
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, f: f64) -> Vec3 {
    Vec3 { e:[self.e[0] / f, self.e[1] / f, self.e[2] / f]}
  }
}

#[allow(dead_code)]
impl Vec3 {
  pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
    Vec3 { e: [e0, e1, e2]}
  }

  pub fn x(&self) -> f64 {
    self.e[0]
  }
  pub fn y(&self) -> f64 {
    self.e[1]
  }
  pub fn z(&self) -> f64 {
    self.e[0]
  }
  pub fn r(&self) -> f64 {
    self.e[0]
  }
  pub fn g(&self) -> f64 {
    self.e[1]
  }
  pub fn b(&self) -> f64 {
    self.e[0]
  }

  pub fn length(&self) -> f64 {
   (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
  }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v/v.length()
}