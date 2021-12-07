use std::collections::HashMap;

fn tick(fish_tank: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_tank: HashMap<i64, i64> = HashMap::new();
    for (key, value) in &fish_tank{
        if key == &0{
            *new_tank.entry(8).or_insert(0) += value;
            *new_tank.entry(6).or_insert(0) += value;
        } else {
            *new_tank.entry(key-1).or_insert(0) += value;
        }
    }
    new_tank
}

fn part1(input: Vec<i64>, days: i64) -> i64 {
    let mut fish_tank: HashMap<i64, i64> = HashMap::new();
    for value in input {
        *fish_tank.entry(value).or_insert(0) += 1;
    }
    for _x in 0..days {
        fish_tank = tick(fish_tank);
    }
    let mut total = 0;
    for (_key, value) in &fish_tank {
        total += value;
    }
    total
}

fn main() {
    let input = "2,1,1,4,4,1,3,4,2,4,2,1,1,4,3,5,1,1,5,1,1,5,4,5,4,1,5,1,3,1,4,2,3,2,1,2,5,5,2,3,1,2,3,3,1,4,3,1,1,1,1,5,2,1,1,1,5,3,3,2,1,4,1,1,1,3,1,1,5,5,1,4,4,4,4,5,1,5,1,1,5,5,2,2,5,4,1,5,4,1,4,1,1,1,1,5,3,2,4,1,1,1,4,4,1,2,1,1,5,2,1,1,1,4,4,4,4,3,3,1,1,5,1,5,2,1,4,1,2,4,4,4,4,2,2,2,4,4,4,2,1,5,5,2,1,1,1,4,4,1,4,2,3,3,3,3,3,5,4,1,5,1,4,5,5,1,1,1,4,1,2,4,4,1,2,3,3,3,3,5,1,4,2,5,5,2,1,1,1,1,3,3,1,1,2,3,2,5,4,2,1,1,2,2,2,1,3,1,5,4,1,1,5,3,3,2,2,3,1,1,1,1,2,4,2,2,5,1,2,4,2,1,1,3,2,5,5,3,1,3,3,1,4,1,1,5,5,1,5,4,1,1,1,1,2,3,3,1,2,3,1,5,1,3,1,1,3,1,1,1,1,1,1,5,1,1,5,5,2,1,1,5,2,4,5,5,1,1,5,1,5,5,1,1,3,3,1,1,3,1";
    let values1: Vec<i64> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let values2: Vec<i64> = input.split(",").map(|x| x.parse().unwrap()).collect();

    println!("{:?}", part1(values1, 80));
    println!("{:?}", part1(values2, 256));
}
