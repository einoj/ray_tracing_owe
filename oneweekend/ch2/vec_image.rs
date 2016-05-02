mod Vec3;

fn main() {
  let nx = 200;
  let ny = 100;
  let v = Vec3::new(3.14,2.71,4.2);
  println!("P3\n{} {} {}\n",v.e[0],v.e[1],v.e[2]);
  println!("P3\n{} {}\n255",nx,ny);
  //jlet col = vec3{e[0]:
//  for j in (0..ny).rev(){
//    for i in 0..nx {
//      let r: f32 = (i as f32)/(nx as f32);
//      let g: f32 = (j as f32)/(ny as f32);
//      let b: f32 = 0.2;
//      let ir = (255.99*r) as i32;
//      let ig = (255.99*g) as i32;
//      let ib = (255.99*b) as i32;
//      println!("{} {} {}",ir,ig,ib);
//    }
//  }
}
