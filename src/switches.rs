use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    m: usize,
  }
  
  let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
  for i in 0..m {
    input! {
      nn: usize,
      switches: [Usize1;nn]
    }
    map.insert(i, switches.into_iter().collect::<Vec<usize>>());
  }
  
  input! {
    needs: [usize;m]
  }
  
  let mut total = 0;
  for mut i in 0..2usize.pow(n as u32) {
    let mut ons: Vec<usize> = vec![0;n];
    let mut index = 0;
    while 0 <  i {
      ons[index] = i & 1;
      index += 1;
      i >>= 1;
    }
    
    let mut flag = true;
    for key in map.keys() {
      let mut sum = 0;
      for ii in map.get(key).unwrap() {
        sum += ons[*ii];
      }
      if sum % 2 != needs[*key] {
        flag = false;
        break
      }
    }
    
    if flag {
      total += 1;
    }
  }
  println!("{}", total);
}