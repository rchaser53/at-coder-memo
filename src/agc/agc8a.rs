/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    x:isize,
    y:isize
  }

  let ax = x.abs();
  let ay = y.abs();
  let base = (ay - ax).abs();
  if y == x {
    println!("0");
    return
  } else if ax == ay {
    println!("1");
    return
  }
  
  if y == 0 {
    // x:-4 y:0
    if x < 0 {
      println!("{}", base);
    }
    // x:4 y:0
    else {
      println!("{}", base + 1);
    }
    
  } else if y < 0 {
    // x:-4 y:-3
    if x <= y {
      println!("{}", base);
    } else {
      
      if ax < ay {
        // x:-2 y:-3
        if x < 0 {
          println!("{}", base + 2);
        }
        // x:2 y:-3
        else {
          println!("{}", base + 1);
        }
      } else {
        // x:-4 y:-3
        if x < 0 {
          println!("{}", base);
        }
        // x:4 y:-3
        else {
          println!("{}", base + 1);
        }
      }
    }
  }
  else {
    // x:5 y:3
    if y < x {
      println!("{}", base+2);
    }
    else {
      if ax < ay {
        // x:-1, y:3
        if x < 0 {
          println!("{}", base + 1);
        }
        // x:1, y:3
        else {
          println!("{}", base);
        }
      }
      else {
        // x:-4, y:3
        if x < 0 {
          println!("{}", base + 1);
        }
        // x:4, y:3
        else {
          println!("{}", base);
        }
      }
      
    }
  }
}
