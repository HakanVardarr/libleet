use super::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        (0..nums.len() - 1)
            .rev()
            .fold((nums.len() - 1, false), |len, i| {
                if i + nums[i] as usize >= len.0 {
                    (i, true)
                } else {
                    (len.0, false)
                }
            })
            .1
    }
}
