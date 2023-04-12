mod dynamic_programing;

pub struct Solution;

mod test {
    use crate::Solution;

    #[test]
    fn question509() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn question1137() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
