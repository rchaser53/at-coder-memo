use proconio::input;
 
struct Helper {
  r: usize,
  s: usize,
  p: usize
}
 
impl Helper {
  fn get_score(&self, c: char) -> usize {
    match c {
      'r' => self.r,
      's' => self.s,
      'p' => self.p,
      _ => panic!("should not come here"),
    }
  }
}
 
fn main() {
  input! {
    n: usize,
    k: usize,
    r: usize,
    s: usize,
    p: usize,
    t: String
  }
  
  let helper = Helper { r: p, s: r, p: s };  
  let t: Vec<char> = t.chars().collect();
  let mut count = 0;
  let never_match_val = 'a';
  for i in 0..k {
    let mut index = i;
    let mut s_count = 0;
    let mut last_val = never_match_val;
    while index < n {
      let c = t[index];
      let val = helper.get_score(c);
      if last_val != c {
        s_count += val;
        last_val = c;
      } else {
        last_val = never_match_val;
      }
      index += k;
    }
 
    count += s_count;
  }
 
  println!("{}", count);
}