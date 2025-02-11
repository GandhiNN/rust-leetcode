pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/* Write a function that checks if a given array of elevation
points follows a "mountain" structure: An increasing sequence
followed by a strictly decreasing one. Return 1 if it follows
the mountain structure, or 0 if it does not.

Example (1)
Input: path = [1, 3, 5, 4, 2]
Output: 1 # (The path rises to a peak at 5 and then descends)

Example (2)
Input: path = [1, 2, 3, 4, 5]
Output: 0 # (No descent after the peak)

Note: No plateaus are allowed (i.e. equal adjacent numbers)
*/
pub fn mountain_path_v1(path: &[i32]) -> i32 {
    let mut state: Vec<String> = vec![];
    for i in 0..path.len() - 1 {
        if path[i] < path[i + 1] {
            state.push("up".to_string());
            if i > 0 {
                if state[i - 1] == "down" {
                    return 0;
                }
            }
        } else if path[i] > path[i + 1] {
            state.push("down".to_string());
        } else {
            return 0;
        }
    }
    if !state.contains(&"down".to_string()) {
        return 0;
    }
    1
}

pub fn mountain_path_v2(path: &[i32]) -> i32 {
    let mut state: Vec<String> = vec![];
    for i in 0..path.len() - 1 {
        match path[i].cmp(&path[i + 1]) {
            std::cmp::Ordering::Less => {
                state.push("up".to_string());
                if i > 0 && state[i - 1] == "down" {
                    return 0;
                }
            }
            std::cmp::Ordering::Greater => {
                state.push("down".to_string());
            }
            std::cmp::Ordering::Equal => {
                return 0;
            }
        }
    }
    if !state.contains(&"down".to_string()) {
        return 0;
    }
    1
}

/* Two pointers: if one people traverse the mountain
from the left and there is another people traverse the
mountain from the right, they will have the same "peak" */
pub fn mountain_path_v3(path: &[i32]) -> i32 {
    let mut peak_left = 0;
    let mut peak_right = 0;
    let mountain_len = path.len();
    // array must contain at least 3 elements
    if mountain_len < 3 {
        return 0;
    }
    // traverse from the left
    for i in 1..mountain_len - 1 {
        if (path[i] > path[i - 1]) && (path[i] > path[i + 1]) {
            peak_left = path[i];
            println!("peak left: {}", peak_left);
            break;
        } else if (path[i + 1] == path[i]) || (path[i] < path[i - 1]) {
            return 0;
        }
    }
    // traverse from the right
    for (i, v) in path.iter().enumerate().rev().skip(1) {
        if (path[i] > path[i - 1]) && (path[i] > path[i + 1]) {
            peak_right = *v;
            println!("peak right: {}", peak_right);
            break;
        } else if (path[i + 1] == path[i]) || (path[i] < path[i - 1]) {
            return 0;
        }
    }
    if peak_left == peak_right { 1 } else { 0 }
}
// traverse from the right

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mountain_path_true() {
        let paths = vec![1, 3, 5, 4, 2];
        let expected = 1;
        assert_eq!(mountain_path_v3(&paths), expected);
    }

    #[test]
    fn test_mountain_path_false_1() {
        let paths = vec![1, 2, 3, 4, 5];
        let expected = 0;
        assert_eq!(mountain_path_v3(&paths), expected);
    }

    #[test]
    fn test_mountain_path_false_2() {
        let paths = vec![2, 1, 2, 3, 4, 3, 1];
        let expected = 0;
        assert_eq!(mountain_path_v3(&paths), expected);
    }

    #[test]
    fn test_mountain_path_with_plateau() {
        let paths = vec![1, 2, 2, 4, 3, 1];
        let expected = 0;
        assert_eq!(mountain_path_v3(&paths), expected);
    }
}
