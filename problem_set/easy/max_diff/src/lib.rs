/* Given a list of integers, find the maximum
difference without using built-in `min`, `max`
and `sort` functions */

pub fn max_diff(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums.len() - 1 {
        for j in (i + 1)..nums.len() {
            let diff = nums[i] - nums[j];
            if diff.abs() > res {
                res = diff.abs();
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_diff() {
        let nums_1: Vec<i32> = vec![0, 1, 2, 3];
        let expected_1 = 3;
        let result_1 = max_diff(nums_1);
        assert_eq!(result_1, expected_1);

        let nums_2: Vec<i32> = vec![-1, -2, -3];
        let expected_2 = 2; // -1 - (-3)
        let result_2 = max_diff(nums_2);
        assert_eq!(result_2, expected_2);
    }
}
