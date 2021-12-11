use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

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
            &hashmap,
            values.len().try_into().unwrap(),
            values[0].len().try_into().unwrap()
        )
    );
    println!(
        "{:?}",
        part2(
            &mut hashmap,
            values.len().try_into().unwrap(),
            values[0].len().try_into().unwrap()
        )
    );
}

fn part1(hashmap: &HashMap<(isize, isize), isize>, rows: isize, cols: isize) -> isize {
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

fn part2(hashmap: &mut HashMap<(isize, isize), isize>, rows: isize, cols: isize) -> i32 {
    let mut basins: Vec<i32> = Vec::new();
    let adder_options: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            let mut deque: VecDeque<(isize, isize)> = VecDeque::new();
            let this_value: &isize = hashmap.get(&(row, col)).unwrap();
            if !visited.contains(&(row, col)) && *this_value != 9 {
                deque.push_back((row, col));
            }
            let mut basin_size = 0;
            while deque.len() > 0 {
                let current_position = deque.pop_front().unwrap();
                for adders in &adder_options {
                    let new_point = (current_position.0 + adders.0, current_position.1 + adders.1);
                    let adjacent_value = hashmap.get(&new_point);
                    if adjacent_value != None
                        && *adjacent_value.unwrap() != 9
                        && !visited.contains(&new_point)
                    {
                        deque.push_back(new_point);
                        visited.insert(new_point);
                        basin_size += 1;
                    }
                }
            }
            if basin_size != 0 {
                basins.push(basin_size);
            }
        }
    }
    basins.sort();
    basins.reverse();
    basins[..3].iter().fold(1, |acc, x| acc * x)
}
