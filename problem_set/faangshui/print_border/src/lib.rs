/* Write a function to create and print a border with
the given width (w) and height (h). The border must follow
a strict pattern:

- Use + for the corners
- Use - for the horizontal lines
- Use | for the vertical lines.

Example Input
w = 6
h = 4

Example Output
+----+
|    |
|    |
+----+
*/

pub fn print_border(w: i32, h: i32) {
    for height in 0..h {
        for width in 0..w {
            if height == 0 || height == h - 1 {
                if width == 0 || width == w - 1 {
                    print!("+");
                } else {
                    print!("-");
                }
            } else if width == 0 || width == w - 1 {
                print!("|");
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
    fn test_print_border() {
        let width = 6;
        let height = 4;

        print_border(width, height);
    }
}
