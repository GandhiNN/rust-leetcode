/* Write a function that, given three lists of food
items (appetizers, mains and desserts), returns all
unique meal combinations. Each meal consists of one
item from each category.

Example input:
appetizers = ["Burger", "Hot Dog"]
mains = ["Fries"]
desserts = ["Soda", "Shake"]

Example output:
[["Burger", "Fries", "Soda"],
["Burger", "Fries", "Shake"],
["Hot Dog", "Fries", "Soda"],
["Hot Dog", "Fries", "Shake"]]
*/
pub fn meal_combinations(
    appetizers: Vec<String>,
    mains: Vec<String>,
    desserts: Vec<String>,
) -> Vec<Vec<String>> {
    // Use nested loop
    let mut combinations: Vec<Vec<String>> = vec![];
    for a in appetizers.iter() {
        for m in mains.iter() {
            for d in desserts.iter() {
                combinations.push(vec![String::from(a), String::from(m), String::from(d)]);
            }
        }
    }
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meal_combinations() {
        let appetizers: Vec<String> = vec!["Burger".into(), "Hot Dog".into()];
        let mains: Vec<String> = vec!["Fries".into()];
        let desserts: Vec<String> = vec!["Soda".into(), "Shake".into()];
        let expected: Vec<Vec<String>> = vec![
            vec!["Burger".into(), "Fries".into(), "Soda".into()],
            vec!["Burger".into(), "Fries".into(), "Shake".into()],
            vec!["Hot Dog".into(), "Fries".into(), "Soda".into()],
            vec!["Hot Dog".into(), "Fries".into(), "Shake".into()],
        ];
        assert_eq!(meal_combinations(appetizers, mains, desserts), expected);
    }
}
