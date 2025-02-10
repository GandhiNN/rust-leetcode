use std::collections::HashSet;

/* Given an integer array `nums`, return `true`
if any value appears more than once in the array,
otherwise (every element is distinct), return `false`

Example 1:
Input: nums = [1, 2, 3, 3]
Output: true

Example 2:
Input: nums = [1, 2, 3, 4]
Output: false
*/
pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    for i in nums.iter() {
        if seen.contains(i) {
            return true;
        } else {
            seen.insert(*i);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        let numlist = vec![1, 2, 3, 3];
        let expected = true;
        assert_eq!(has_duplicate(numlist), expected);
    }

    #[test]
    fn test_has_duplicate_failed() {
        let numlist = vec![1, 2, 3, 4];
        let expected = false;
        assert_eq!(has_duplicate(numlist), expected);
    }

    #[test]
    fn test_has_duplicate_empty() {
        let numlist = vec![];
        let expected = false;
        assert_eq!(has_duplicate(numlist), expected);
    }
}
