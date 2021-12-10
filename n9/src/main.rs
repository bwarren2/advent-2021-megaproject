use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut hashmap: HashMap<(isize, isize), isize> = HashMap::new();

    let values: Vec<Vec<isize>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, x)| {
            let numbers: Vec<isize> = x
                .split("")
                .filter(|x| *x != "")
                .enumerate()
                .map(|(col_idx, d)| {
                    let digit: isize = d.parse().unwrap();
                    hashmap.insert(
                        (row_idx.try_into().unwrap(), col_idx.try_into().unwrap()),
                        digit,
                    );
                    digit
                })
                .collect();
            numbers
        })
        .collect();

    println!(
        "{:?}",
        part1(
            hashmap,
            values.len().try_into().unwrap(),
            values[0].len().try_into().unwrap()
        )
    );
}

fn part1(hashmap: HashMap<(isize, isize), isize>, rows: isize, cols: isize) -> isize {
    println!("{:?}", (rows, cols));

    let mut total = 0;
    let adder_options: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for row in 0..rows {
        for col in 0..cols {
            let mut is_min = true;
            for adders in &adder_options {
                let new_point = (row + adders.0, col + adders.1);
                if hashmap.get(&new_point) != None
                    && hashmap.get(&(row, col)) >= hashmap.get(&new_point)
                {
                    is_min = false;
                }
            }
            if is_min {
                total = total + hashmap.get(&(row, col)).unwrap() + 1;
            }
        }
    }
    total
}
