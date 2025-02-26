/* Write a function to print a right-angled triangle of '*'
characters, where the first row has one '*', the second row
has two '*', and so on, up to the given number of rows.

For example, if the input is `3`, the output should be:

*
**
***

*/
pub fn print_apprentice_steps(rows: &i32) {
    for i in 1..=*rows {
        for _j in 1..i + 1 {
            print!("*");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_apprentice_steps() {
        let rows = 3;
        print_apprentice_steps(&rows);
    }
}
