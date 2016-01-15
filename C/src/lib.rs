
extern crate animal;
extern crate D;
use animal::Animal;
use std::any::Any;
use D::D_MSG;
use D::Dog;

pub fn foo() -> &'static str {
  return D_MSG;
}

pub struct Cat {
    pub c: u8
}

pub fn bar(a:Box<Any>) {
  D::bar(a);
}


pub fn get_dog() -> Box<animal::Animal>
{
  Box::new(Dog {d: 42})
}

pub fn get_animal() -> Box<Any>
{
  Box::new(Dog {d: 90})
}


//pub fn say_hi<T:animal::Animal>(an_animal:Box<T>)
pub fn say_hi(an_animal:Box<animal::Animal>)
//pub fn say_hi(an_animal:Box<Dog>)
{
  println!("[C]: {}", an_animal.speak());
}
