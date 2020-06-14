use vec3::dot;

fn hit_sphere(center: vec3::Vec3, radius: f64, r: &ray::Ray) -> bool {
    let oc: vec3::Vec3 = r.origin() - center;
    let  a: f64 = dot(&r.direction(), &r.direction());
    let b: f64 = 2.0 * dot(&oc, &r.direction());
    let c: f64 = dot(&oc, &oc) - radius*radius;
    let discriminant: f64 = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn color(r: &ray::Ray) -> vec3::Vec3 {
    if hit_sphere(vec3::Vec3::new(0.0,0.0,-1.0), 0.5, r) {
	return vec3::Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t:f64 = 0.5*(unit_direction.y()+1.0);
    vec3::Vec3::new(1.0,1.0,1.0)*(1.0-t) + vec3::Vec3::new(0.5, 0.7, 1.0)*t
}

fn main() {
  let nx: i32 = 400;
  let ny: i32 = 200;
  print!("P3\n{} {}\n255\n",nx,ny);
  let lower_left_corner = vec3::Vec3::new(-2.0,-1.0,-1.0);
  let horizontal = vec3::Vec3::new(4.0,0.0,0.0);
  let vertical = vec3::Vec3::new(0.0,2.0,0.0);
  let origin = vec3::Vec3::new(0.0,0.0,0.0);
  for j in (0..ny-1).rev() {
    for i in 0..nx {
      let u: f64 = (i as f64)/(nx as f64);
      let v: f64 = (j as f64)/(ny as f64);
      let r = ray::Ray::new(&origin,&(lower_left_corner+horizontal*u+vertical*v));
      let col: vec3::Vec3 = color(&r);
      let ir = (255.99*col[0]) as i32;
      let ig = (255.99*col[1]) as i32;
      let ib = (255.99*col[2]) as i32;
      println!("{} {} {}",ir,ig,ib);
    }
  }
}
    
