/* Write a function that, given a list of bamboo
stick strenghts and a threshold, returns the number
of sticks greater than the threshold

Example Input:
sticks = [12, 7, 15, 10, 6]
threshold = 8

Example Output:
3
*/
pub fn count_sturdy_sticks(sticks: Vec<i32>, threshold: i32) -> i32 {
    let res = sticks.iter().filter(|&x| x > &threshold).count();
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sturdy_sticks() {
        let sticks = [vec![12, 7, 15, 10, 6], vec![20, 25, 30], vec![]];
        let thresholds = [8, 15, 5];
        let expected = [3, 3, 0];
        for i in 0..sticks.len() - 1 {
            assert_eq!(
                count_sturdy_sticks(sticks[i].clone(), thresholds[i]),
                expected[i]
            );
        }
    }
}
