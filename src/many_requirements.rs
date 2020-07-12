use proconio::input;
use proconio::marker::Usize1;
 
struct ToCulc<'a> {
  N: usize,
  M: usize,
  Q: usize,
  A: &'a [(usize, usize, usize, usize)],
  max: usize
}
 
impl <'a>ToCulc<'a> {
  pub fn culc(&mut self, inner_stack: &mut Vec<usize>) {
    if inner_stack.len() == self.N + 1 {
      let mut temp = 0;
      for j in self.A.iter() {
        if inner_stack[j.1] - inner_stack[j.0] == j.2 {
          temp += j.3;
        }
      }
      self.max = std::cmp::max(self.max, temp);
      return
    }
    
    let last_len = inner_stack.len() - 1;
    while inner_stack[last_len] <= self.M {
      let mut new_stack = inner_stack.clone();
      new_stack.push(*new_stack.last().unwrap());
      self.culc(&mut new_stack);
      inner_stack[last_len] += 1;
    }
  }
}
 
fn main() {
  input! {
    N: usize,
    M: usize,
    Q: usize,
    A: [(Usize1, Usize1, usize, usize);Q]
  }
  
  let mut to_culc = ToCulc {
    N, M, Q, A: &A, max: 0
  };
  
  let mut stack: Vec<usize> = vec![1];
  to_culc.culc(&mut stack);
  
  println!("{}", to_culc.max);
}