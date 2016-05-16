extern crate vec3;

fn main() {
  let v1 = vec3::Vec3::new(3.14,2.71,4.2);
  let v2 = vec3::Vec3::new(3.14,2.71,4.2);

  let v3 = v1+v2;

  println!("{} {} {}",v3.e[0], v3.e[1], v3.e[2]);
}
