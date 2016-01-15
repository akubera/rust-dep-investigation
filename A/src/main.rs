
extern crate B;
extern crate C;
extern crate animal;

use B::Balloon;
use C::Cat;



fn main()
{
  let balloon = Balloon { b: 90};   
//  println!("Hello, world! ({})", balloon.b);
//  println!("B: {}", B::foo());
//  println!("C: {}", C::foo());
//  println!("B dog: {}", B::get_dog().speak());
//  println!("C dog: {}", C::get_dog().speak());

//  B::say_hi(C::get_dog());
//  C::say_hi(B::get_dog());


  B::bar(C::get_animal());

}
