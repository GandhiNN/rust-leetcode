/* Write a function to prompt the user for a name
and use nested loops to search for it in a 2D list of
names.

Specification:
-> Print "Found" if the name is present
-> Print "Not Found" otherwise

Example input and output:
names = [
    ["Alice", "Bob"],
    ["Cara", "Dan"]
]

name_to_search = "Cara"

Output: Found
*/
pub fn search_name(names: &Vec<Vec<String>>, name_to_search: &str) -> String {
    for outer in names {
        for inner in outer {
            if inner == name_to_search {
                return String::from("Found");
            }
        }
    }
    String::from("Not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_name() {
        let names: Vec<Vec<String>> = vec![vec!["Alice".into(), "Bob".into()], vec![
            "Cara".into(),
            "Dan".into(),
        ]];
        let name_to_search = "Cara";
        assert_eq!(search_name(&names, name_to_search), "Found");
    }
}
