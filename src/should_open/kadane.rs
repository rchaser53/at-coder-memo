// Kadane's Algorithm
// 連続する部分配列で最大値を求めるやつ。O(N)で解ける
// 連続する部分配列が負ならば無視して良いという考え
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
  let mut max = nums[0];
  let mut temp = 0;
  for v in nums {
    if temp < 0 {
      temp = v;
    } else {
      temp += v;
    }
    max = std::cmp::max(max, temp);
  }
  
  max
}