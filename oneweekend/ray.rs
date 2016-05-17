extern crate vec3;

pub struct Ray {
  origin: vec3::Vec3,
  direction: vec3::Vec3,
}

impl Ray {

  pub fn origin(self) -> vec3::Vec3 {
    self.origin
  }

  pub fn direction(self) -> vec3::Vec3 {
    self.direction
  }

  pub fn point_at_parameter(self, t: f64) -> vec3::Vec3 {
    self.origin + self.direction*t
  }
}

