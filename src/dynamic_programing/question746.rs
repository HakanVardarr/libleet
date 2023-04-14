use super::Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for c in cost {
            let temp = a;
            a = b;
            b = c + temp.min(b);
        }

        a.min(b)
    }
}
