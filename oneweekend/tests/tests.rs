extern crate vec3;
extern crate ray;

fn main() {
  let v1 = vec3::Vec3::new(3.14,2.71,4.2);
  let v2 = vec3::Vec3::new(3.14,2.71,4.2);

  let mut v3 = v1+v2;
  let v1 = vec3::Vec3::new(3.14,2.71,4.2);
  let v2 = vec3::Vec3::new(3.14,2.71,4.2);
  let v4 = v1*v2;

  let v1 = vec3::Vec3::new(3.14,2.71,4.2);
  let v2 = vec3::Vec3::new(3.14,2.71,4.2);
  let r = ray::Ray{origin: v1, direction: v2};

  println!("length is {}", v3.length());
  println!("V3 is {} {} {}",v3.e[0], v3.e[1], v3.e[2]);
  v3 = v3/2.0;
  println!("V3/2 is {} {} {}",v3.e[0], v3.e[1], v3.e[2]);
  println!("{} {} {}",v4.e[0], v4.e[1], v4.e[2]);
  
  println!("{} {} {}",r.origin.e[0], r.origin.e[1], r.origin.e[2]);
  r.point_at_parameter(4.4);
  //println!("{} {} {}",r.origin.e[0], r.origin.e[1], r.origin.e[2]);
}
