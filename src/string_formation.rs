use std::collections::VecDeque;
 
fn main() {
  let mut char_vec: VecDeque<char> = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned().chars().collect()
  };
  
  let Q = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned().parse::<usize>().unwrap()
  };
  
  let mut flag = true;
  for _ in 0..Q {
    let s = {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      s.trim_end().to_owned()
    };
 
    let ws: Vec<&str> = s.split_whitespace().collect();
    if ws.len() == 1 {
       flag = !flag;
    } else {
       if ws[1] == "1" {
         if flag {
           char_vec.push_front(ws[2].chars().next().unwrap());
         } else {
           char_vec.push_back(ws[2].chars().next().unwrap());         
         }
       } else {
         if flag {
           char_vec.push_back(ws[2].chars().next().unwrap());
         } else {
           char_vec.push_front(ws[2].chars().next().unwrap());
         }
       };
    }
  }

  let mut a: Vec<char> = char_vec.into_iter().collect();
  if !flag {
    a.reverse();
  }
  let a: String = a.iter().collect();
  println!("{}", a);
}