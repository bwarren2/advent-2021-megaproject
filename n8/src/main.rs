use std::collections::HashSet;

#[derive(Debug)]
struct Line<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl Line<'_> {
    fn from_str(string: &str) -> Line {
        let sides: Vec<&str> = string.split("|").collect();
        let inputs = sides[0]
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| *s != "")
            .collect();
        let outputs = sides[1]
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| *s != "")
            .collect();
        Line { inputs, outputs }
    }
    fn easy_digit_count(&self) -> i32 {
        let set: HashSet<usize> = [2, 3, 4, 7].iter().cloned().collect();
        let easy_count = self
            .outputs
            .iter()
            .filter(|s| set.contains(&s.len()))
            .collect::<Vec<&&str>>()
            .len()
            .try_into()
            .unwrap();
        println!("{:?}", (&self.outputs, easy_count));
        easy_count
    }
}

fn main() {
    let string = include_str!("input.txt");
    let lines: Vec<Line> = string.lines().map(|s| Line::from_str(s)).collect();
    println!(
        "{:?}",
        lines.iter().fold(0, |acc, x| acc + x.easy_digit_count())
    );
}
