// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ops::*;
use ops::sequence::Ordering::*;
use std::ops::{Deref, DerefMut};

pub struct Vector<T>
{
  vec: Vec<T>
}

impl<T> Vector<T>
{
  pub fn wrap(vec: Vec<T>) -> Vector<T> {
    Vector{vec: vec}
  }
}

impl<T> Deref for Vector<T>
{
  type Target = Vec<T>;

  fn deref<'a>(&'a self) -> &'a Vec<T> {
    &self.vec
  }
}

impl<T> DerefMut for Vector<T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut Vec<T> {
    &mut self.vec
  }
}

impl<T> Empty for Vector<T> {
  fn empty() -> Vector<T> {
    vec![]
  }
}

impl<T> Singleton<T> for Vector<T> {
  fn singleton(value: T) -> Vector<T> {
    vec![value]
  }
}

impl<T> Push<Back, T> for Vector<T> {
  fn push(&mut self, value: T) {
    self.push(value);
  }
}

impl<T> Pop<Back, T> for Vector<T> {
  fn pop(&mut self) -> Option<T> {
    self.pop()
  }
}
