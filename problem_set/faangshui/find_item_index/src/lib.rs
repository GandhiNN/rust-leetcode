/* Write a function that searches for a given
item in an array and returns its index

_Example input_
inventory = ["Lantern", "Rope", "Map", "Compass"]
item = "Map"

_Example output_
2

Note: Return -1 if item not found in the inventory
*/
pub fn find_item_index(inventory: &[String], item: &str) -> i32 {
    for (idx, val) in inventory.iter().enumerate() {
        if item == val {
            return idx as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_item_index_found() {
        let inventory: [String; 4] = [
            "Lantern".into(),
            "Rope".into(),
            "Map".into(),
            "Compass".into(),
        ];
        let item = "Map";
        let expected = 2;
        assert_eq!(find_item_index(&inventory, item), expected);
    }

    #[test]
    fn test_find_item_index_not_found() {
        let inventory: [String; 4] = [
            "Lantern".into(),
            "Rope".into(),
            "Map".into(),
            "Compass".into(),
        ];
        let item = "Car";
        let expected = -1;
        assert_eq!(find_item_index(&inventory, item), expected);
    }
}
