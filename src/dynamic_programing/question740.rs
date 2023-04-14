use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut count = std::collections::HashMap::new();
        for num in nums.clone() {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut nums = nums
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();

        nums.sort();

        let mut earn1 = 0;
        let mut earn2 = 0;
        for i in 0..nums.len() {
            let cur = nums[i] * count.get(&(nums[i] as i32)).unwrap();
            if i > 0 && nums[i] == nums[i - 1] + 1 {
                let temp = earn2;
                earn2 = earn2.max(cur + earn1);
                earn1 = temp;
            } else {
                let temp = earn2;
                earn2 = cur + earn2;
                earn1 = temp;
            }
        }

        earn2
    }
}
