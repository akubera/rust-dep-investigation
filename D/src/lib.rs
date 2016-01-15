
extern crate animal;

use std::any::Any;

pub const D_MSG: &'static str = "Bow Wow (D1/B)!";

pub struct Dog {
    pub d: u8
}

// impl Any for Dog {
// }


impl animal::Animal for Dog {
  fn speak(&self) -> &'static str { 
    let dog = Box::new(Dog {d:1});
    bar(dog);
    D_MSG
  }
}

// pub fn bar(an_animal:Box<animal::Animal>)
// pub fn bar(an_animal:Box<Dog>)
// pub fn bar<T:animal::Animal>(an_animal:Box<T>)

pub fn bar(an_animal:Box<Any>)
{
//  let foo:Box<Any> = an_animal;
//  if an_animal.is::<animal::Animal>() {

  if an_animal.is::<Dog>() {
    println!("[bar] is dog!!!!");
  } else {
    println!("[bar] is not dog.");
  }
}

