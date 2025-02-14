/* Write a function to identify unique elements
present in a collection, ordered by hierarchy.

The hierarchy is: Hopeful -> Seeker -> Wayfinder
-> Seer -> Master -> Elder.

Solve this using a loop and a few conditionals, no
need to use a set.

Example input:
ranks = ["Wayfinder", "Hopeful", "Seeker", "Hopeful", "Master", "Hopeful", "Elder"]

Example output:
[Hopeful, Seeker, Wayfinder, Master, Elder]
*/
pub fn unique_ranks_in_order(ranks: Vec<String>) -> Vec<String> {
    // Define the result container
    let mut result: Vec<String> = vec![];
    // Define the rank sequence
    let sequence: Vec<String> = vec![
        "Hopeful".into(),
        "Seeker".into(),
        "Wayfinder".into(),
        "Seer".into(),
        "Master".into(),
        "Elder".into(),
    ];
    // Get the ranks seen in the input
    let mut seen: Vec<String> = vec![];
    for rank in ranks.iter() {
        if !seen.contains(rank) {
            seen.push(rank.into());
        }
    }
    // Iterate through the rank sequence agains the ranks seen
    for seq in sequence.iter() {
        if seen.contains(seq) {
            result.push(seq.into());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_ranks_in_order() {
        let ranks: Vec<String> = vec![
            "Wayfinder".into(),
            "Hopeful".into(),
            "Seeker".into(),
            "Hopeful".into(),
            "Master".into(),
            "Hopeful".into(),
            "Elder".into(),
        ];
        let expected = vec!["Hopeful", "Seeker", "Wayfinder", "Master", "Elder"];
        assert_eq!(unique_ranks_in_order(ranks), expected);
    }
}
