pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn min_hall(halls: Vec<i32>) -> i32 {
    let mut res: i32 = -1; // anchor value because number of guests cannot be negative
    for h in halls {
        if res < 0 {
            res = h
        };
        if h < res {
            res = h;
        }
    }
    res
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
    fn test_min_hall() {
        let input1: Vec<i32> = vec![10, 20, 8, 1, 4, 100, 23, 20];
        let expected1: i32 = 1;
        assert_eq!(expected1, min_hall(input1));

        let input2: Vec<i32> = vec![3, 4, 5, 2, 3, 10, 21, 17];
        let expected2: i32 = 2;
        assert_eq!(expected2, min_hall(input2));
    }
}
