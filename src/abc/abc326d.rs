/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
  use proconio::input;
  use proconio::marker::*;
  use std::cmp::Ordering;
  use std::collections::*;

  pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
  }
  
  impl<T> LexicalPermutation for [T] where T: Ord {
    fn next_permutation(&mut self) -> bool {
      if self.len() < 2 { return false; }
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] >= self[i] {
        i -= 1;
      }
  
      if i == 0 {
        return false;
      }
  
      let mut j = self.len() - 1;
      while j >= i && self[j] <= self[i-1]  {
        j -= 1;
      }
  
      self.swap(j, i-1);
  
      self[i..].reverse();
  
      true
    }
  }

 fn convert(a: char) -> usize {
  if a == '.' {
    0
  } else {
    1 << (a as usize - 'A' as usize)
  }
 }

  fn print_helper(memo: &mut Vec<usize>, p: &Vec<Vec<char>>) {
    for i in 0..memo.len() {
      let i = memo[i];
      println!("{}", p[i].iter().map(|v| v.to_string()).collect::<String>());
    }
    // println!("");
  }

  struct Helper {
    rd: Vec<char>,
    cd: Vec<char>,
    success: Option<Vec<usize>>,
  }

  impl Helper {
    fn dfs(&mut self, memo: &mut Vec<usize>, p: &Vec<Vec<char>>, cols: &mut Vec<usize>) {
      let n = self.rd.len();
      let ci = memo.len();
      if n == ci {
        // print_helper(memo, p);
        for v in cols {
          if *v != 7 {
            return
          }
        }

        self.success = Some(memo.clone());
        return
      }

      for pi in 0..p.len() {
        memo.push(pi);
        let ni = memo.len() - 1;
        let pattern = &p[pi];

        // rd(横)
        let mut fc = ' ';
        for i in 0..n {
          if pattern[i] != '.' {
            fc = pattern[i];
            break
          }
        }

        if fc != self.rd[ni] {
          memo.pop();
          continue
        }

        // cd(縦)
        let mut success = true;
        for coi in 0..n {
          let cc = p[pi][coi];
          if cc == '.' {
            continue
          }
          let cv = convert(cc);          
          let should_c = self.cd[coi];

          if cols[coi] == 0 {
            if cc != should_c {
              success = false;
              break
            }
          } else {
            if cols[coi] & cv == cv {
              success = false;
              break
            }
          }

        }

        if success {
          for coi in 0..n {
            cols[coi] += convert(p[pi][coi]);
          }
          self.dfs(memo, p, cols);
          for coi in 0..n {
            cols[coi] -= convert(p[pi][coi]);
          }
        }
        memo.pop();
      }
    }
  }
  
  fn main() {
    input! {
      n:usize,
      rd:Chars,
      cd:Chars,
    }

    let mut dict = "ABC".to_string().chars().into_iter().collect::<Vec<char>>();
    for _ in dict.len()..n {
      dict.push('.');
    }
    dict.sort();
    let mut set = HashSet::new();

    loop {
      set.insert(dict.clone());
      let _ = dict.iter();
      if !dict.next_permutation() {
        break
      }
    }
    let mut dict = set.into_iter().collect::<Vec<Vec<char>>>();
    dict.sort();
    
    let mut helper = Helper { rd, cd, success: None };
    helper.dfs(&mut vec![], &dict, &mut vec![0;n]);

    if let Some(arr) = helper.success {
      println!("Yes");
      print_helper(&mut arr.clone(), &dict);
    } else {
      println!("No");
    }
  }