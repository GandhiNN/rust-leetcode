/* Write a function that generates and returns a sequence
for a given starting number `n` with the following properties:
- If the number is even, it's divided by 2
- If the number is odd, it's multiplied by 3 and incremented by 1.
The sequence ends when the number reaches 1.

Example Input:
start = 6

Example Output:
[6, 3, 10, 5, 16, 8, 4, 2, 1]
*/
pub fn meditation_sequence(starting_number: &i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![*starting_number];
    let mut sn = *starting_number;
    while sn != 1 {
        if sn % 2 == 0 {
            // even number
            sn /= 2;
        } else if sn % 2 == 1 {
            // odd number
            sn = (sn * 3) + 1;
        }
        res.push(sn);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meditation_sequence() {
        let starting = 6;
        let expected = vec![6, 3, 10, 5, 16, 8, 4, 2, 1];
        assert_eq!(meditation_sequence(&starting), expected);
    }
}
