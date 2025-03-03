/* Create a function to print this
diamond using '*' characters, given an
odd number `n` that specifies the width
and height.

Example input:
n = 5

Expected output:
  *
 ***
*****
 ***
  *

*/
pub fn print_diamond(n: &i32) {
    // n must be an odd number
    if n % 2 == 0 {
        return;
    }
    // a diamond has n x n dimension
    // where middle index has n number of *
    for i in 1..*n + 1 {
        let mut j = i - (*n / 2 + 1);
        if j < 0 {
            j *= -1;
        }
        // diamond print logic
        print!("{}", " ".repeat(j as usize));
        print!("{}", "*".repeat((*n - (j * 2)) as usize));
        print!("{}", " ".repeat(j as usize));
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_diamond_5() {
        print_diamond(&5);
    }

    #[test]
    fn test_print_diamond_9() {
        print_diamond(&9);
    }
}
