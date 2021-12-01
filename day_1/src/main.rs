use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_numbers();
    let counter = evaluate_sum(lines);

    println!("{}", counter);
}

fn evaluate(lines: Vec<i32>) -> i32 {
    let mut previuous_line: i32 = 0;
    let mut counter: i32 = 0;
    for measure in lines {
        if previuous_line <= 0 {
            previuous_line = measure.clone();
        }
        if measure > previuous_line {
            counter = counter + 1;
        }
        previuous_line = measure.clone();
    }
    counter
}

fn evaluate_sum(lines: Vec<i32>) -> i32 {
    let sums: Vec<i32> = lines
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();
    evaluate(sums)
}

fn read_numbers() -> Vec<i32> {
    let file = File::open::<_>("input").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|item| item.expect("Not a number"))
        .map(|number| number.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_case() {
        let counter = evaluate(vec![1, 2]);
        assert_eq!(1, counter);
    }

    #[test]
    fn easy_case_2() {
        let counter = evaluate(vec![1, 2, 3, 5]);
        assert_eq!(3, counter);
    }

    #[test]
    fn easy_case_3() {
        let counter = evaluate(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(7, counter);
    }
    #[test]
    fn create_sliding_window() {
        let counter = evaluate_sum(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(5, counter);
    }
}
