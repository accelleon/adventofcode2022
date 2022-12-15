fn calc_contain(input: &str) -> i32 {
    // Each line is 2 ranges e.g 1-3,2-4
    // Calculate the number of pairs of lines that completely contain another
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let mut ranges = line.split(',');
            let range1 = ranges.next().unwrap();
            let range2 = ranges.next().unwrap();
            let mut range1 = range1.split('-');
            let mut range2 = range2.split('-');
            let range1_start = range1.next().unwrap().parse::<i32>().unwrap();
            let range1_end = range1.next().unwrap().parse::<i32>().unwrap();
            let range2_start = range2.next().unwrap().parse::<i32>().unwrap();
            let range2_end = range2.next().unwrap().parse::<i32>().unwrap();
            (range1_start >= range2_start && range1_end <= range2_end) ||
            (range2_start >= range1_start && range2_end <= range1_end)
        })
        .count() as i32
}

fn calc_overlap(input: &str) -> i32 {
    // Each line is 2 ranges e.g 1-3,2-4
    // Calculate the number of pairs of lines that overlap
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let mut ranges = line.split(',');
            let range1 = ranges.next().unwrap();
            let range2 = ranges.next().unwrap();
            let mut range1 = range1.split('-');
            let mut range2 = range2.split('-');
            let range1_start = range1.next().unwrap().parse::<i32>().unwrap();
            let range1_end = range1.next().unwrap().parse::<i32>().unwrap();
            let range2_start = range2.next().unwrap().parse::<i32>().unwrap();
            let range2_end = range2.next().unwrap().parse::<i32>().unwrap();
            (range1_start >= range2_start && range1_start <= range2_end) ||
            (range1_end >= range2_start && range1_end <= range2_end) ||
            (range2_start >= range1_start && range2_start <= range1_end) ||
            (range2_end >= range1_start && range2_end <= range1_end)
        })
        .count() as i32
}

fn main() {
    // Read input from input.txt
    let input = include_str!("input.txt");
    println!("{}", calc_contain(&input));
    println!("{}", calc_overlap(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_contain() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        assert_eq!(calc_contain(input), 2);
    }

    #[test]
    fn test_calc_overlap() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        assert_eq!(calc_overlap(input), 4);
    }
}
