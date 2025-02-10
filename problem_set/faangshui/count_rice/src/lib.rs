pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn count_rice(rice_bags: Vec<i32>) -> i32 {
    let mut total_bag = 0;
    for bag in rice_bags {
        total_bag += bag
    }
    total_bag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_rice() {
        let res = count_rice(vec![10, 20, 11, 22]);
        assert_eq!(res, 63);
    }
}
