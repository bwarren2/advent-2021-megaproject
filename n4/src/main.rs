use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    cells: [[isize; 5]; 5],
}

impl Board {
    #[allow(dead_code)]
    fn has_win(&self, called_numbers: &HashSet<isize>) -> bool {
        for row in 0..5 {
            let row_values = HashSet::from(self.cells[row]);
            if row_values
                .intersection(called_numbers)
                .collect::<HashSet<&isize>>()
                .len()
                == row_values.len()
            {
                return true;
            }
        }
        for col in 0..5 {
            let col_values: [isize; 5] = self
                .cells
                .iter()
                .map(|x| x[col])
                .collect::<Vec<isize>>()
                .try_into()
                .unwrap();
            let col_set = HashSet::from(col_values);
            if col_set
                .intersection(called_numbers)
                .collect::<HashSet<&isize>>()
                .len()
                == col_set.len()
            {
                return true;
            }
        }
        false
    }

    fn from_str(input: String) -> Self {
        Board {
            cells: input
                .lines()
                .map(|r| {
                    let temp_arr: [isize; 5] = r
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<isize>>()
                        .try_into()
                        .unwrap();
                    temp_arr
                })
                .collect::<Vec<[isize; 5]>>()
                .try_into()
                .unwrap(),
        }
    }

    fn score(&self, called_numbers: &HashSet<isize>) -> isize {
        let mut score: isize = 0;
        for row in 0..5{
            for col in 0..5{
                let value: isize = self.cells[row][col];
                if !called_numbers.contains(&value){
                    score += value;
                }
            }
        }
        score 
    }


}

fn part1(boards: &mut Vec<Board>, draws: &Vec<isize>)-> isize{
    let mut draws_so_far: HashSet<isize> = HashSet::new();
    for draw in draws{
        draws_so_far.insert(*draw);
        for board in boards.iter(){
            if board.has_win(&draws_so_far){
                return board.score(&draws_so_far) * draw;
            }
        }
    }
    -1
}

fn part2(boards: &mut Vec<Board>, draws: &Vec<isize>)-> isize{
    let mut draws_so_far: HashSet<isize> = HashSet::new();
    let mut winners: HashSet<usize> = HashSet::new();
    for draw in draws{
        draws_so_far.insert(*draw);
        for (idx, board) in boards.iter().enumerate(){
            if board.has_win(&draws_so_far){
                winners.insert(idx);
                if winners.len() == boards.len(){
                    return board.score(&draws_so_far) * draw;
                }
            }
        }
    }
    -1
}


fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let draws  = lines[0].split(",").map(|num| num.parse().unwrap()).collect::<Vec<isize>>();
    let mut boards: Vec<Board> = Vec::new();
    let mut board_string: String = "".to_string();
    for line in lines[1..].iter(){
        if line == &""{
            if board_string != "".to_string(){
                boards.push(Board::from_str(board_string));    
            } 
            board_string = "".to_string();
        } else {
            board_string = board_string + line + "\n";
        }
    }
    
    println!("{:?}", part1(&mut boards, &draws));
    println!("{:?}", part2(&mut boards, &draws));
    // println!("{:?}", board.has_win(&HashSet::from([57, 8, 17, 51])));
}
