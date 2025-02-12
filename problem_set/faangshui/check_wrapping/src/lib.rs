/* Write a function to check if a given string
of W and S symbols represents a properly wrapped
and sealed set of boxes.

The order is crucial: We can't seal a box before it's wrapped
and every wrap must have a corresponding seal in the correct
sequence. In other words, 'S' cannot precedes 'W' but
'W' can be followed by another 'W' or 'S

i.e. "WWS" is valid but "WSW" is not.

Return "Perfectly wrapped!" if the boxes are in the
correct order, or "Wrapping error!" if there's a mistake.

Example input (1) -> boxes = "WWSS"
Example output (1) -> "Perfectly wrapped!"
Explanation: All 'S'es are 'wrapped' by 'W'es

Example input (2) -> boxes = "WSW"
Example output (2) -> "Wrapping error!"
Explanation: there's an 'S' followed by 'W'
*/
pub fn check_wrapping(boxes: &str) -> String {
    // minimum length of string is 2
    if boxes.len() == 1 {
        return String::from("Wrapping error!");
    }
    // set a counter variable
    let mut counter = 0;
    for i in 0..boxes.len() {
        // when counter is 0, that means that we don't have any "free" 'W' available
        if counter == 0 && boxes.chars().nth(i).unwrap() == 'S' {
            return String::from("Wrapping error!");
        } else if boxes.chars().nth(i).unwrap() == 'W' {
            // increment counter if 'W'
            counter += 1;
        } else {
            // decrement counter if 'S'
            counter -= 1;
        }
    }
    if counter == 0 {
        String::from("Perfectly wrapped!")
    } else {
        String::from("Wrapping error!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_wrapping_1() {
        let boxes = "WWSS";
        let expected = "Perfectly wrapped!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_2() {
        let boxes = "WSW";
        let expected = "Wrapping error!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_3() {
        let boxes = "WS";
        let expected = "Perfectly wrapped!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_4() {
        let boxes = "W";
        let expected = "Wrapping error!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_5() {
        let boxes = "S";
        let expected = "Wrapping error!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_6() {
        let boxes = "WWSWSS";
        let expected = "Perfectly wrapped!";
        assert_eq!(check_wrapping(boxes), expected);
    }

    #[test]
    fn test_check_wrapping_7() {
        let boxes = "WSS";
        let expected = "Wrapping error!";
        assert_eq!(check_wrapping(boxes), expected);
    }
}
