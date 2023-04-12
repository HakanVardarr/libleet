use std::collections::HashMap;

// Question 1137. N-th Tribonacci Number

//The Tribonacci sequence Tn is defined as follows:
// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
// Given n, return the value of Tn.

use super::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 0);
        map.insert(1, 1);
        map.insert(2, 1);

        Solution::_tribonacci(n, &mut map)
    }

    fn _tribonacci(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(num) = map.get(&n) {
            return *num;
        } else {
            let result = Solution::_tribonacci(n - 3, map)
                + Solution::_tribonacci(n - 2, map)
                + Solution::_tribonacci(n - 1, map);

            map.insert(n, result);
            result
        }
    }
}
