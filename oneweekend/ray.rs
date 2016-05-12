extern crate vec3;

pub struct Ray {
  A: vec3::Vec3,
  B: vec3::Vec3,
}

impl Ray {

  pub fn origin(&self) -> vec3::Vec3 {
    self.A
  }
}
