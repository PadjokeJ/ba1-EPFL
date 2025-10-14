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
    let mx = 10_000;

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
    bubble(&mut vec);
    println!("dt bubble : {}", now.elapsed().as_millis());

    /*
    vec = shuffled_vec(mx);

    let now = Instant::now();
    bogo(&mut vec);
    println!("dt bogo : {}", now.elapsed().as_millis());
    */
}
