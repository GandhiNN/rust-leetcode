/* Write a function that, given a list of charm
sizes, returns the product of all charms with odd
sizes. If no odd-sized charms are present, return 1 */
pub fn lucky_charms_product(charms: Vec<i32>) -> i32 {
    let res = charms
        .iter()
        .filter(|&x| x % 2 != 0)
        .fold(1, |acc, &x| acc * x);
    res
}

// Better version of the code, avoid unecessary fold clippy warnings!
pub fn lucky_charms_product_v2(charms: Vec<i32>) -> i32 {
    charms.iter().filter(|&x| x % 2 != 0).product::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lucky_charms_product() {
        let charms_1 = vec![3, 6, 5, 10, 7];
        let expected_1 = 105;
        assert_eq!(lucky_charms_product(charms_1), expected_1);

        let charms_2 = vec![2, 4, 6];
        let expected_2 = 1;
        assert_eq!(lucky_charms_product(charms_2), expected_2);
    }

    #[test]
    fn test_lucky_charms_product_v2() {
        let charms_1 = vec![3, 6, 5, 10, 7];
        let expected_1 = 105;
        assert_eq!(lucky_charms_product_v2(charms_1), expected_1);

        let charms_2 = vec![2, 4, 6];
        let expected_2 = 1;
        assert_eq!(lucky_charms_product_v2(charms_2), expected_2);
    }
}
