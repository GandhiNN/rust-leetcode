/* Given a list representing lanterns where `1`
indicates the lantern is lit and `-1` indicates
the lantern is unlit, write a function to count
how many are lit using an accumulator. */

pub fn count_lit_lanterns(lanterns: Vec<i32>) -> i32 {
    let mut lit_count = 0;
    for lantern in lanterns.into_iter() {
        if lantern == 1 {
            lit_count += 1;
        }
    }
    lit_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lit_lanterns() {
        let lanterns: Vec<i32> = vec![-1, 1, 1, -1, 1, 1, 1, -1];
        let expected = 5;
        let result = count_lit_lanterns(lanterns);
        assert_eq!(result, expected);
    }
}
