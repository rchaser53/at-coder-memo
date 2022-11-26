use std::collections::*;

fn helper(que: &mut VecDeque<usize>, indices: &mut Vec<i32>, nums: &Vec<i32>, i:usize) {
    while let Some(&top_i) = que.back() {
        if nums[top_i] <= nums[i] {
            break
        }
        indices[top_i] = i as i32;
        que.pop_back();
    }
    que.push_back(i);
}

// 単調スタック(monotonic_stack monotonic stack)
pub fn monotonic_stack_problem(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut que = VecDeque::with_capacity(n);
    let right = {
        let mut temp = vec![n as i32;n];
        for i in 0..n {
            helper(&mut que, &mut temp, &nums, i);
        }
        temp
    };

    que.clear();
    let left = {
        let mut temp = vec![-1;n];
        for i in (0..n).rev() {
            helper(&mut que, &mut temp, &nums, i);
        }
        temp
    };

    let mut result = vec![0;n];
    for i in 0..n {
        let len = right[i] - left[i] - 1;
        let ti = len as usize - 1;
        result[ti] = result[ti].max(nums[i]);
    }

    for i in (0..n-1).rev() {
        result[i] = result[i].max(result[i+1]);
    }
    result
}
