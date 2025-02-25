/* Write a function that takes a single argument `seats_per_row`
and prints a seating plan for the cinema with the given number
of seats per row. Use '#' to represent a seat.

Note 1: the function does not need to return anything.
Note 2: the number of rows is fixed at 3

_Example Input and Output_
input = 7
output = "#######
#######
#######
"
*/
pub fn print_seating_plan(seats_per_row: i32) {
    for _j in 0..3 {
        for i in 0..seats_per_row {
            print!("#");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_seating_plan() {
        print_seating_plan(4);
    }
}
