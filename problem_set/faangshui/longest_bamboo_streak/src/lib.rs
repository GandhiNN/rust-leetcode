pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
/* Write a function that, given an array
of integers, returns the length of the longest
streak of consecutive elements with increasing
sequence.

Example input (1)  -> heights = [3, 4, 2, 5, 6, 3, 7, 8, 9]
Example output (1) -> 4
Explanation -> the longest streak of increasing numbers is [3, 7, 8, 9] with length of 4

Example input (2) -> heights - [5, 4, 3, 2, 1]
Example output (2) -> 1
Explanation -> there is no increasing streak, so the longest subarray length is 1
*/
pub fn longest_bamboo_streak(heights: &Vec<i32>) -> i32 {
    let mut counter: Vec<i32> = vec![];
    let mut longest = 1;
    for i in 0..heights.len() {
        if i != 0 {
            if heights[i] > heights[i - 1] {
                counter.push(heights[i]);
            } else if heights[i] < heights[i - 1] {
                if counter.len() > longest {
                    longest = counter.len();
                    counter = vec![heights[i]]; // reset the counter
                }
            }
        } else {
            counter.push(heights[i]);
        }
    }
    // println!("{:?} => {}", counter, counter.len());
    if counter.len() > longest {
        longest = counter.len();
    }
    longest as i32
}

pub fn longest_bamboo_streak_v2(heights: &[i32]) -> i32 {
    let mut counter: Vec<i32> = vec![];
    let mut longest = 1;
    for i in 0..heights.len() {
        if i != 0 {
            match heights[i].cmp(&heights[i - 1]) {
                std::cmp::Ordering::Greater => counter.push(heights[i]),
                std::cmp::Ordering::Less => {
                    if counter.len() > longest {
                        longest = counter.len();
                        counter = vec![heights[i]]; // reset the counter
                    }
                }
                std::cmp::Ordering::Equal => (),
            }
        } else {
            counter.push(heights[i]);
        }
    }
    if counter.len() > longest {
        longest = counter.len();
    }
    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_bamboo_streak_1() {
        let heights = vec![3, 4, 2, 5, 6, 3, 7, 8, 9];
        let expected = 4;
        assert_eq!(longest_bamboo_streak_v2(&heights), expected);
    }

    #[test]
    fn test_longest_bamboo_streak_2() {
        let heights = vec![1, 2, 3, 4, 5];
        let expected = 5;
        assert_eq!(longest_bamboo_streak_v2(&heights), expected);
    }

    #[test]
    fn test_longest_bamboo_streak_3() {
        let heights = vec![5, 4, 3, 2, 1];
        let expected = 1;
        assert_eq!(longest_bamboo_streak_v2(&heights), expected);
    }
}
