mod dynamic_programing;
mod question71;

pub struct Solution;

mod test {

    #[allow(unused)]
    use crate::Solution;

    #[test]
    fn question509() {
        // assert_eq!(Solution::fib(2), 1);
        // assert_eq!(Solution::fib(3), 2);
        // assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn question1137() {
        // assert_eq!(Solution::tribonacci(25), 1389537);
    }

    #[test]
    fn question71() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );

        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());

        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
    }

    #[test]
    fn question70() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn question746() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn question198() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![1, 1]), 1);
    }

    #[test]
    fn question213() {
        assert_eq!(Solution::rob2(vec![1, 1, 1]), 1);
    }

    #[test]
    fn question740() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }
}
