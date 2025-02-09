/* Write a function to check if input vector
is sorted in ascending order. If it's perfectly
oredered, return "Perfectly aligned, Chef!". If
it's out of order, return "Someone's messing with
my ingredients!" */
pub fn check_ingredients(ingredients: &[i32]) -> String {
    let mut sorted = false;
    if ingredients.len() > 1 {
        for (idx, val) in ingredients.iter().enumerate() {
            if idx == ingredients.len() - 1 {
                break;
            }
            if val <= &ingredients[idx + 1] {
                sorted = true;
                continue;
            } else {
                sorted = false;
                break;
            }
        }
    } else {
        sorted = true;
    }
    if sorted {
        return String::from("Perfectly aligned, Chef!");
    } else {
        return String::from("Someone's messing with my ingredients!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ingredients_succeed() {
        let ingredients = vec![1, 2, 3, 4, 5];
        let expected = "Perfectly aligned, Chef!";
        assert_eq!(check_ingredients(&ingredients), expected);
    }

    #[test]
    fn test_check_ingredients_failed() {
        let ingredients = vec![1, 2, 1, 4, 3];
        let expected = "Someone's messing with my ingredients!";
        assert_eq!(check_ingredients(&ingredients), expected);
    }

    #[test]
    fn test_check_ingredients_all_elements_are_the_same() {
        let ingredients = vec![1, 1, 1, 1, 1];
        let expected = "Perfectly aligned, Chef!";
        assert_eq!(check_ingredients(&ingredients), expected);
    }

    #[test]
    fn test_check_ingredients_one_element_only() {
        let ingredients = vec![5];
        let expected = "Perfectly aligned, Chef!";
        assert_eq!(check_ingredients(&ingredients), expected);
    }
}
