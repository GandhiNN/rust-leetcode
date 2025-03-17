/* Write a function to print a number triangle
of height 'n' where each row starts with its
row number and contains numbers up to that row count

Example Input:
n = 4

Example Output:
1
2 3
3 4 5
4 5 6 7
*/
pub fn print_number_triangle(n: &i32) {
    for i in 1..=*n {
        for j in i..i + i {
            print!("{} ", j);
            if j == i + i - 1 {
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_number_triangle() {
        let n = 4;
        print_number_triangle(&n);
    }
}
