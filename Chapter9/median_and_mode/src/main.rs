use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
  let mut rng = thread_rng();
  let mut v: Vec<i32> = Vec::new();
  let mut i: i32 = 0;

  while i < 99 {
    v.push(rng.gen_range(0..100));
    i += 1;
  }
  
  v.sort();

  println!("{:?}", v);

  let median = median(&v);
  println!("The median is {}.", median);

  let mode = mode(&v);
  println!("The mode is {:?}.", mode);
}

fn median(v: &Vec<i32>) -> f32 {
     
  println!("{:?}", v);

  let v_len = v.len();
  let mut median;

  if (v_len % 2 == 0) {
      median = (v[v_len/2] as f32 + v[v_len/2 - 1] as f32) / 2.0;
  } else {
      median = v[v_len/2] as f32;
  }
  println!("The median of the list is {median}");

  median
}

fn mode(v: &Vec<i32>) -> Vec<i32> {
    let mut mode_hash = HashMap::new();
    let mut modes: Vec<i32> = Vec::new();
    let mut max = 0;

    for &num in v {
        *mode_hash.entry(num).or_insert(0) += 1;
    }

    println!("{:?}", mode_hash);

    //check if number is current max number, if so add to list, if greater clear then add
    for (&k, &v) in mode_hash.iter() {
        if v == max {
            modes.push(k);
        } else if v > max {
            max = v;
            modes.clear();
            modes.push(k);
        } 
    }
    
    modes
}

