#[derive(Copy, Clone)]
pub struct Ray {
  pub origin: vec3::Vec3,
  pub direction: vec3::Vec3,
}

impl Ray {

  pub fn new(a: &vec3::Vec3, b: &vec3::Vec3) -> Ray {
    Ray{origin:*a, direction:*b}
  }

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

