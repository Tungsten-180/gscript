use std::f64::consts::PI;

#[derive(Debug)]
pub struct Settings{
  pub nozzle:f64,
  pub layer_height:f64,
  pub layer_width:f64,
  pub max_extrusion:Option<f64>,
  pub extrusion_mult:f64,
}

pub fn line(set:&Settings,x:&f64,y:&f64)->String{
  //println!("{:?}",set);
  let l = {x-y}.abs();
  let [w,h,n] = [set.layer_width,set.layer_height,set.nozzle];
  let e = ((h*w*l)/(n*n*PI))*set.extrusion_mult;
  //println!("e{:?}:w{:?}:h{:?}:n{:?}:l{:?}:x{:?}:y{:?}",e,w,h,n,l,x,y);
  format!("G1 X{} Y{} E{}\n",x,y,e)
}