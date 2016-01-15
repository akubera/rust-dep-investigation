
extern crate animal;

use std::any::Any;

pub const D_MSG: &'static str = "Bark! (D2/C)";

pub struct Dog {
    pub d: u8
}


impl animal::Animal for Dog {
  fn speak(&self) -> &'static str { D_MSG }
}

pub fn bar(an_animal:Box<Any>)
{
//  let foo:Box<Any> = an_animal;
//  if an_animal.is::<animal::Animal>() {

  if an_animal.is::<Dog>() {
    println!("[D2::bar] is dog!!!!");
  } else {
    println!("[D2::bar] is not dog.");
  }
}

