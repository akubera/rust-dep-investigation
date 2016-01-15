
use std::any::Any;

pub trait Animal : Any {
  fn speak(&self) -> &'static str;
}
