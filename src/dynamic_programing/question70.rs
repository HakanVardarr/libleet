use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);

        if n <= 2 {
            return n;
        } else {
            for i in 3..=n {
                map.insert(i, map.get(&(i - 1)).unwrap() + map.get(&(i - 2)).unwrap());
            }

            return *map.get(&n).unwrap();
        }
    }
}
