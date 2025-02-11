/* Given a non-empty list of integers, return `True`
if there is a place to split the list so that the sum
of the numbers on one side is equal to the sum of the
numbers on the other side.

For example, given the list [1, 5, 3, 3],
the function would return `True`, because we can split
the list so that [1, 5].sum() == [3, 3].sum().

Another example is [7, 3, 4], that would also return
True because [7].sum() == [3, 4].sum()

On the other hand, [1, 2, 2], would return False
because there is no way we can split the list so
it will produce two lists which have the same sum()
*/
pub fn check_and_balance(input: Vec<i32>) -> bool {
    for i in 1..input.len() - 1 {
        let mut vec = input.clone();
        let vec2: Vec<i32> = vec.split_off(i);
        if vec2.iter().sum::<i32>() == vec.iter().sum::<i32>() {
            return true;
        } else {
            continue;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_and_balance() {
        let input = vec![1, 5, 3, 3];
        let expected = true;
        assert_eq!(check_and_balance(input), expected);
    }

    #[test]
    fn test_check_and_balance_v2() {
        let input = vec![7, 3, 4];
        let expected = true;
        assert_eq!(check_and_balance(input), expected);
    }

    #[test]
    fn test_check_and_balance_v3() {
        let input = vec![1, 2, 2];
        let expected = false;
        assert_eq!(check_and_balance(input), expected);
    }
}
