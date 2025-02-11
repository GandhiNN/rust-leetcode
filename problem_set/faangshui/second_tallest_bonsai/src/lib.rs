/* Write a function to find the second largest
element in an array representing bonsai heights
which will contain at least two elements

Example input: [15, 22, 19, 18, 25, 17]
Example output: 22
*/
pub fn second_tallest_bonsai(bonsai_heights: &[i32]) -> i32 {
    let mut tallest = 0;
    let mut second_tallest = 0;
    for height in bonsai_heights.iter() {
        if *height >= tallest {
            second_tallest = tallest;
            tallest = *height;
        } else if *height < tallest && *height > second_tallest {
            second_tallest = *height;
        }
    }
    second_tallest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_tallest_bonsai_1() {
        let bonsai_heights = vec![15, 22, 19, 18, 25, 17];
        let expected = 22;
        assert_eq!(second_tallest_bonsai(&bonsai_heights), expected);
    }

    #[test]
    fn test_second_tallest_bonsai_2() {
        let bonsai_heights = vec![5, 5, 5];
        let expected = 5;
        assert_eq!(second_tallest_bonsai(&bonsai_heights), expected);
    }

    #[test]
    fn test_second_tallest_bonsai_3() {
        let bonsai_heights = vec![1000, 1000, 999, 999, 998, 998, 997, 997, 996, 996];
        let expected = 1000;
        assert_eq!(second_tallest_bonsai(&bonsai_heights), expected);
    }

    #[test]
    fn test_second_tallest_bonsai_4() {
        let bonsai_heights = vec![2147483646, 2147483645, 2147483644, 2147483643, 2147483642];
        let expected = 2147483645;
        assert_eq!(second_tallest_bonsai(&bonsai_heights), expected);
    }
}
