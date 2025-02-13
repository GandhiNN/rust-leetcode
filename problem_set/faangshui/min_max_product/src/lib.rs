/* Write a function that, given an array of
distinct weights, returns both the minimum and
maximum products of any two distinct weights

Example (1)
input -> weights = [3, 5, 1, 7]
output -> (3, 35)
explanation -> the minimum product is 1*3 = 3 and
    the maximum product is 5*7 = 35

Example (2)
input -> weights = [10, -5, -10, -3]
output -> (-100, 30)
explanation -> the minimum product is -10*10 = -100
    and the maximum product is 10*3 = 30
*/
pub fn min_max_product(weights: &Vec<i32>) -> (i32, i32) {
    let mut highest: i32 = -99999999; // sentinel value
    let mut lowest: i32 = 99999999; // sentinel value
    for w in 0..weights.len() {
        for i in w + 1..weights.len() {
            let product = weights[w] * weights[i];
            if product > highest {
                highest = product;
            }
            if product < lowest {
                lowest = product;
            }
        }
    }
    (lowest, highest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_product_1() {
        let weights = vec![3, 5, 1, 7];
        let expected = (3, 35);
        assert_eq!(min_max_product(&weights), expected);
    }

    #[test]
    fn test_min_max_product_2() {
        let weights = vec![10, -5, -10, -3];
        let expected = (-100, 30);
        assert_eq!(min_max_product(&weights), expected);
    }
}
