use super::Solution;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        for i in (0..nums.len() - 2).rev() {
            let mut largest = 0;

            for k in i + 2..nums.len() {
                if nums[k] > largest {
                    largest = nums[k];
                }
            }
            nums[i] += largest as i32;
        }

        nums.sort();
        nums[nums.len() - 1]
    }
}
