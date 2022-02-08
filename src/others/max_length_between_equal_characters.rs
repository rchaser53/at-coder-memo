impl Solution {
  pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut memo = vec![vec![];26];
    let s = s.chars().into_iter().collect::<Vec<char>>();
    for i in 0..s.len() {
        let ci = (s[i] as u8 - 'a' as u8) as usize;
        memo[ci].push(i as i32);
    }
    let mut result = -1;
    for arr in &memo {
        if 1 < arr.len() {
          result = std::cmp::max(arr[arr.len()-1] - arr[0] - 1, result);
        }
    }
    result
  }
  }