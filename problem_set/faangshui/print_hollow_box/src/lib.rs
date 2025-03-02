/* Write a function to print a rectangular, hollow
box with dimensions 'w' (width) and 'h' (height)
using '*' for the border

Example input and output:
input: w = 4, h = 3;
output:
****
*  *
****
*/
pub fn print_hollow_box(w: i32, h: i32) {
    for i in 1..=h {
        for j in 1..=w {
            if i == 1 || i == h {
                print!("*");
            } else if (i != 1 || i != h) && (j == 1 || j == w) {
                print!("*")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hollow_box() {
        let width = 4;
        let height = 3;
        print_hollow_box(width, height);
    }
}
