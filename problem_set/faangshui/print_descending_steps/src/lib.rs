/* Write a function to print an inverted right-angled triangle
of '*' characters, where the first row has as many '*' as the
total number of rows, the second row has one less, and so on,
until there's just one '*'.

Note: Use a nested loop to achieve this instead of multiplying
the '*' character

_Example Input and Output_
Input = 3

Output =
***
**
*

*/
pub fn print_descending_steps(rows: &i32) {
    let mut n: i32 = *rows;
    for _i in 0..*rows {
        for _j in 0..n {
            print!("*");
        }
        n -= 1;
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_descending_steps() {
        let rows = 3;
        print_descending_steps(&rows);
    }
}
