/* Write a function that, given a list of energy values,
returns the difference between the sum of the even and odd
numbers (even sum - odd sum)

Example input = [2, 5, 8, 3, 10]
Example output = 12

Explanation :
> even sum = 2 + 8 + 10 = 20
> odd sum = 5 + 3 = 8
> diff = 20 - 8 = 12
*/
pub fn stone_energy_difference(stones: &[i32]) -> i32 {
    let mut even_sum = 0;
    let mut odd_sum = 0;
    if stones.len() == 0 {
        return 0;
    }
    for stone in stones.iter() {
        if stone % 2 != 0 {
            odd_sum += stone;
        } else {
            even_sum += stone;
        }
    }
    even_sum - odd_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_energy_difference() {
        let stones = [2, 5, 8, 3, 10];
        let expected = 12;
        assert_eq!(stone_energy_difference(&stones), expected);
    }

    #[test]
    fn test_stone_energy_difference_no_even() {
        let stones = [1, 3, 3];
        let expected = -7;
        assert_eq!(stone_energy_difference(&stones), expected);
    }

    #[test]
    fn test_stone_energy_difference_no_odd() {
        let stones = [2, 4, 6];
        let expected = 12;
        assert_eq!(stone_energy_difference(&stones), expected);
    }

    #[test]
    fn test_stone_energy_difference_no_element() {
        let stones = [];
        let expected = 0;
        assert_eq!(stone_energy_difference(&stones), expected);
    }
}
