use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:String
  }

  if s == String::from("zyxwvutsrqponmlkjihgfedcba") {
    println!("-1");
    return
  }

  let mut s = s.chars().collect::<Vec<char>>();
  let n = s.len();
  if n == 26 {
    let mut set = HashSet::new();
    for &c in &s {
      set.insert(c);
    }

    for i in (0..26).rev() {
      let c = s[i];
      set.remove(&c);

      for j in 0..26 {
        let v = ('a' as u8 + j) as char;
        if set.contains(&v) { continue }
        if v <= c { continue }
        let mut result = String::from("");
        for k in 0..i {
          result = format!("{}{}", result, s[k as usize]);
        }
        result = format!("{}{}", result, v);
        println!("{}", result);
        return
      }
      
      
    }
  } else {
    for i in 0..26 {
      let v = ('a' as u8 + i) as char;
      if !s.contains(&v) {
        s.push(v);
        println!("{}", s.into_iter().map(|v| v.to_string()).collect::<String>());
        return
      }
    }
  }
  
}