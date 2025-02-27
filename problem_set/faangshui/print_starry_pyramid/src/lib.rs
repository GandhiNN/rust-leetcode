/* Write a function to print a pattern of '*'
for a given odd number of rows `n` where:
- the first row has 1 star.
- each subsequent row adds 1 star until reaching the midpoint.
- after the midpoint, the rows decrease in start symmetrically.

Example Input and Output:
input = 5
output =
*
**
***
**
*

*/
pub fn print_starry_pyramid(n: &i32) {
    // if n < 1 or n is an even number, exit the program
    if (*n < 1) || (*n % 2 == 0) {
        println!("Invalid input");
    } else {
        // get the midpoint index
        // use floor division
        let midpoint = n / 2;
        for _i in 0..=midpoint {
            for _j in 0..=_i {
                print!("*");
            }
            println!()
        }
        let mut diff = n - midpoint - 1;
        for _y in midpoint..=*n {
            for _z in 0..diff {
                print!("*");
            }
            diff -= 1;
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_starry_pyramid() {
        let input = 13;
        print_starry_pyramid(&input);
    }
}
