use super::Solution;

impl Solution {
    pub fn rob2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let a = Solution::_rob(nums[..nums.len() - 1].to_vec());
        let b = Solution::_rob(nums[1..].to_vec());

        a.max(b)
    }

    fn _rob(mut nums: Vec<i32>) -> i32 {
        for i in (0..nums.len() - 2).rev() {
            let mut largest = 0;

            for k in i + 2..nums.len() {
                println!("{k}");
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
