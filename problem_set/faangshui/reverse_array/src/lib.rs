/* Write a function that returns a new array,
which is the reverse of the original array using
a loop

_Example Input_
scrolls = ["Scroll A", "Scroll B", "Scroll C", "Scroll D"]

_Example Output_
["Scroll D", "Scroll C", "Scroll B", "Scroll A"]
*/
pub fn reverse_array(array: &[String]) -> Vec<String> {
    array
        .iter()
        .rev()
        .map(|x| x.into())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_array() {
        let input_array: &[String] = &[
            String::from("Scroll A"),
            String::from("Scroll B"),
            String::from("Scroll C"),
            String::from("Scroll D"),
        ];
        let expected: Vec<String> = vec![
            String::from("Scroll D"),
            String::from("Scroll C"),
            String::from("Scroll B"),
            String::from("Scroll A"),
        ];
        assert_eq!(reverse_array(input_array), expected);
    }
}
