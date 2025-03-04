/* Write a function to print a multiplication
table for numbers 1 through n (where n <= 9)
using nested loops.

Example Input:
n = 5

Expected Output:
1 2 3 4 5
2 4 6 8 10
3 6 9 12 15
4 8 12 16 20
5 10 15 20 25
*/
pub fn print_multiplication_table(n: &i32) {
    if *n > 9 {
        return;
    }
    for row in 1..=*n {
        for col in 1..=*n {
            print!("{} ", col * row);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_multiplication_table() {
        print_multiplication_table(&5);
    }

    #[test]
    fn test_print_multiplication_table_2() {
        print_multiplication_table(&9);
    }
}
