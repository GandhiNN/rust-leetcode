use std::collections::HashMap;

/// Intuition
/// <!-- Describe your first thoughts on how to solve this problem. -->

/// Approach
/// <!-- Describe your approach to solving the problem. -->

/// Complexity
/// Time complexity:
/// <!-- Add your time complexity here, e.g. $$O(n)$$ -->

/// Space complexity:
/// <!-- Add your space complexity here, e.g. $$O(n)$$ -->

// Using brute force i.e. nested iteration -> O(n^2)
pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution: Vec<i32> = vec![];
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                solution = vec![i as i32, j as i32];
            }
        }
    }
    solution
}

// Using double-pass hashmap
pub fn two_sum_double_pass_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // First pass: create a hashmap where
    // index of the vector as the key and
    // value of the vector as the value
    let num_map: HashMap<usize, i32> = nums.clone().into_iter().enumerate().collect();
    // Second pass: iterate the original vector again
    // and check if the complement is stored as value in the hashmap
    for (idx, val) in nums.iter().enumerate() {
        // Check for complement: target - current value
        let complement = target - val;
        // Check if complement exists in the num_map
        let second_idx =
            num_map.clone().into_iter().find_map(
                |(key, val)| {
                    if val == complement { Some(key) } else { None }
                },
            );
        if second_idx.is_some() && second_idx.unwrap() != idx {
            return vec![idx as i32, second_idx.unwrap() as i32];
        }
    }
    // Solution not found
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_brute_force() {
        let r1 = two_sum_brute_force(vec![2, 7, 11, 15], 9);
        assert_eq!(r1, vec![0, 1]);

        let r2 = two_sum_brute_force(vec![3, 2, 4], 6);
        assert_eq!(r2, vec![1, 2]);

        let r3 = two_sum_brute_force(vec![3, 3], 6);
        assert_eq!(r3, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_double_pass_hashmap() {
        let r1 = two_sum_double_pass_hashmap(vec![2, 7, 11, 15], 9);
        assert_eq!(r1, vec![0, 1]);
    }
}
