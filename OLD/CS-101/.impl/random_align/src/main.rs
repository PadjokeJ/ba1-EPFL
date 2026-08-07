extern crate rand;

use crate::rand::Rng;
use rand::rng;
use rand::seq::SliceRandom;

fn gen_shuffled_vec(max: i32, digit: i32) -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::with_capacity(max as usize);
  let mut rnd = rng();
  
  println!("{}", vec.capacity());
  for i in 0..vec.capacity() {
    vec.push(rnd.random::<i32>() % digit);
  }

  println!("{}", vec.len());

  let vec = vec;
  vec
}

fn search_repeating(vec: Vec<i32>) {
  let mut prev = vec[0];
  let mut repeats = 0;
  let mut max_repeats = 0;
  for i in 1..vec.len() {
    prev = vec[i - 1];
    if prev == vec[i] {
      repeats += 1;
    } else {
      if max_repeats < repeats {
        max_repeats = repeats;
      }
      repeats = 0;
    }
  }
      
  println!("max : {}", max_repeats);
}

fn main() {
  for i in 0..100 {
    let vec = gen_shuffled_vec(1_000_000, 2);
    search_repeating(vec);
  }
}
