

fn most_calories(stream: &str) -> i32 {
    // stream is a string of numbers separated by newlines
    // individual elf inventories is separated by two newlines

    let calories_total = stream.split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .map(|ing| ing.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        });

    // return the caloric total of the inventory with the most calories
    calories_total.max().unwrap()
}

fn top_three_sum(stream: &str) -> i32 {
    // stream is a string of numbers separated by newlines
    // individual elf inventories is separated by two newlines

    let calories_total = stream.split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .map(|ing| ing.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        });

    // Return the top 3 values
    let mut top_three = [0, 0, 0];
    for c in calories_total {
        if c > top_three[0] {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = c;
        } else if c > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = c;
        } else if c > top_three[2] {
            top_three[2] = c;
        }
    }
    top_three[0] + top_three[1] + top_three[2]
}

fn main() {
    // Read in input.txt
    let input = include_str!("input.txt");
    println!("{}", most_calories(&input));
    println!("{}", top_three_sum(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input =  "1000\n\
                            2000\n\
                            3000\n\
                            \n\
                            4000\n\
                            \n\
                            5000\n\
                            6000\n\
                            \n\
                            7000\n\
                            8000\n\
                            9000\n\
                            \n\
                            10000";

        assert_eq!(most_calories(input), 24000);
    }

    #[test]
    fn it_works_top_three() {
        let input =  "1000\n\
                            2000\n\
                            3000\n\
                            \n\
                            4000\n\
                            \n\
                            5000\n\
                            6000\n\
                            \n\
                            7000\n\
                            8000\n\
                            9000\n\
                            \n\
                            10000";

        assert_eq!(top_three_sum(input), 24000+11000+10000);
    }
}
