use super::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut l = 0usize;
        let mut r = 0usize;

        while r < nums.len() - 1 {
            let mut f = 0;
            for i in l..r + 1 {
                f = f.max(i + nums[i] as usize);
            }
            l = r + 1;
            r = f;
            result += 1;
        }

        result
    }
}
