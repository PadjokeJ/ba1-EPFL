extern crate rand;

use std::time::Instant;
use rand::rng;
use rand::seq::SliceRandom;

fn shuffled_vec(max: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..max + 1).collect();
    vec.shuffle(&mut rng());
    let vec = vec;
    vec
}

fn is_sorted(vec: &mut Vec<i32>) -> bool {
    for i in 0..(vec.len() - 1) {
        if vec[i] > vec[i + 1] {
            return false;
        }
    }
    true
}

fn merge(vec: &Vec<i32>) -> Vec<i32> {
  let len = vec.len();

  if len < 2 {
    vec.to_vec()
  } else {
    let left = merge(&vec[0..(len / 2)].to_vec());
    let right = merge(&vec[(len / 2)..].to_vec());

    let mut merged: Vec<i32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
      if left[i] < right[j] {
        merged.push(left[i]);
        i += 1;
      } else {
        merged.push(right[j]);
        j += 1;
      }
    }

    while i < left.len() {
      merged.push(left[i]);
      i += 1;
    }

    while j < right.len() {
      merged.push(right[j]);
      j += 1;
    }

    merged
  }
}

fn quick(vec: &Vec<i32>) -> Vec<i32> {
  if vec.len() == 0 {
    return Vec::new();
  }
  let pivot: i32 = vec[0];

  if vec.len() <= 1 {
    vec![pivot]
  } else {
    let mut left:  Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for i in 1..vec.len() {
      if vec[i] < pivot {
        left.push(vec[i]);
      } else if vec[i] > pivot {
        right.push(vec[i]);
      }
    }
    let mut sl = quick(&left);
    let mut sr = quick(&right);
    let mut total: Vec<i32> = sl.clone();
    total.push(pivot);
    total.append(&mut sr);
    total
  }
}

fn bubble(vec: &mut Vec<i32>) {
    let len = vec.len();

    loop {
        let mut sorted: bool = true;
        for i in 0..(len - 1) {
            if vec[i] > vec[i + 1] {
                sorted = false;
                let temp = vec[i];
                vec[i] = vec[i + 1];
                vec[i + 1] = temp;
            }
        }
        if sorted {
            break;
        }
    }
}

fn select(vec: &mut Vec<i32>) {
    for j in 0..vec.len() {
        let mut smallest = vec[j];
        let mut sm_i = j;

        for i in j..vec.len() {
            if vec[i] < smallest {
                smallest = vec[i];
                sm_i = i;
            }
        }
        vec[sm_i] = vec[j];
        vec[j] = smallest;
    }
}

fn insert(vec: &mut Vec<i32>) {
    if vec[0] > vec[1] {
        let temp = vec[0];
        vec[0] = vec[1];
        vec[1] = temp;
    }
    for i in 0..(vec.len() - 1) {
        let mut first = i;
        let second = first + 1;
        let temp = vec[second];
        
        while first > 0 && vec[first] > temp {
            first -= 1;
        }
        if temp > vec[first] {
            first += 1;
        }
        for j in (first..second).rev() {
            vec[j + 1] = vec[j];

        }
        vec[first] = temp;

    }
}

fn bogo(vec: &mut Vec<i32>) {
    loop {
        if is_sorted(vec) {
            break;
        }
        vec.shuffle(&mut rng());
    }
}

fn main() {
    let mx = 1000;

    let mut vec: Vec<i32> = shuffled_vec(mx);

    let now = Instant::now();
    insert(&mut vec);
    println!("dt insert : {}", now.elapsed().as_millis());

    vec = shuffled_vec(mx);
    
    let now = Instant::now();
    select(&mut vec);
    println!("dt select : {}", now.elapsed().as_millis());


    vec = shuffled_vec(mx);
    
    let now = Instant::now();
    let _ = merge(&mut vec);
    println!("dt merge  : {}", now.elapsed().as_millis());

    vec = shuffled_vec(mx);
    
    let now = Instant::now();
    let _ = quick(& vec);
    println!("dt quick  : {}", now.elapsed().as_millis());

    vec = shuffled_vec(mx);
    
    let now = Instant::now();
    bubble(&mut vec);
    println!("dt bubble : {}", now.elapsed().as_millis());

    /*
    vec = shuffled_vec(mx);

    let now = Instant::now();
    bogo(&mut vec);
    println!("dt bogo : {}", now.elapsed().as_millis());
    */
}
