/* A function that returns the index of the hall
with the minimum number of guests. If multiple halls
have the same minimum number, return the one with the
smallest index */
pub fn min_hall_index(halls: Vec<i32>) -> i32 {
    let mut min_val = -1; // halls cannot contain < 0 number of guests
    let mut min_idx = 0;
    for (idx, val) in halls.into_iter().enumerate() {
        if min_val < 0 {
            // initialize the min val with a proper number
            min_val = val;
        };
        if val < min_val {
            min_val = val;
            min_idx = idx;
        }
    }
    min_idx as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_hall_index() {
        let halls: Vec<i32> = vec![10, 12, 1, 3, 4, 2];
        let expected = 2;
        let result = min_hall_index(halls);
        assert_eq!(result, expected);
    }
}
