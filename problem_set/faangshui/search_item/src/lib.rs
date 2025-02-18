/* Write a function to search for an item
and report if it exists in the list. */
pub fn search_item(inventory: &[String], item: &str) -> String {
    for i in inventory.iter() {
        if item == i {
            return String::from("Item exists");
        }
    }
    String::from("Item does not exist")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_item_1() {
        let inventory = [
            String::from("Lantern"),
            String::from("Rope"),
            String::from("Map"),
            String::from("Compass"),
        ];
        let item = "Rope";
        let expected = "Item exists";
        assert_eq!(search_item(&inventory, item), expected);
    }

    #[test]
    fn test_search_item_2() {
        let inventory = [
            String::from("Lantern"),
            String::from("Rope"),
            String::from("Map"),
            String::from("Compass"),
        ];
        let item = "Key";
        let expected = "Item does not exist";
        assert_eq!(search_item(&inventory, item), expected);
    }
}
