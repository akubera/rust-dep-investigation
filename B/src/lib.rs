
extern crate animal;
extern crate D;
use D::Dog;
use animal::Animal;
use std::any::Any;

pub fn foo() -> &'static str {
  return D::D_MSG;
}

pub struct Balloon {
    pub b: u8
}

pub fn bar(a:Box<Any>) {
  D::bar(a);
}


pub fn get_dog() -> Box<animal::Animal>
// pub fn get_dog() -> Box<Dog>
{
  Box::new(Dog {d: 90})
}

pub fn get_animal() -> Box<Any>
// pub fn get_dog() -> Box<Dog>
{
  Box::new(Dog {d: 90})
}

// pub fn say_hi<T:Animal>(an_animal:Box<T>)

// pub fn say_hi(an_animal:Box<Dog>)

pub fn say_hi(an_animal:Box<Animal>)
{
  println!("[B]: {}", an_animal.speak());
}
