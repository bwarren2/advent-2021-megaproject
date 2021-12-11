use phf::phf_map;

#[derive(Debug)]
struct Chunk<'a> {
    letters: Vec<&'a str>,
}

static CLOSURES: phf::Map<&'static str, &'static str> = phf_map! {
    ">" => "<",
    "]" => "[",
    "}" => "{",
    ")" => "(",
};

static OPENERS: phf::Map<&'static str, &'static str> = phf_map! {
    "<" => ">",
    "[" => "]",
    "{" => "}",
    "(" => ")",
};

static SCORES: phf::Map<&'static str, i64> = phf_map! {
    ")" => 3,
    "]" => 57,
    "}" => 1197,
    ">" => 25137,
};

static COMPLETION_SCORES: phf::Map<&'static str, i64> = phf_map! {
    ")" => 1,
    "]" => 2,
    "}" => 3,
    ">" => 4,
};

impl Chunk<'_> {
    fn from_str(input: &str) -> Chunk {
        Chunk {
            letters: input.split("").filter(|x| *x != "").collect(),
        }
    }
    fn illegal_char_value(&self) -> i64 {
        let mut char_stack: Vec<&str> = Vec::new();
        for (idx, symbol) in self.letters.iter().enumerate() {
            if !CLOSURES.contains_key(symbol) {
                char_stack.push(symbol);
            } else {
                if char_stack.last() != CLOSURES.get(symbol) {
                    return *SCORES.get(symbol).unwrap();
                } else {
                    char_stack.pop();
                }
            }
        }
        0
    }

    fn is_corrupt(&self) -> bool {
        let mut char_stack: Vec<&str> = Vec::new();
        for (idx, symbol) in self.letters.iter().enumerate() {
            if !CLOSURES.contains_key(symbol) {
                char_stack.push(symbol);
            } else {
                if char_stack.last() != CLOSURES.get(symbol) {
                    return true;
                } else {
                    char_stack.pop();
                }
            }
        }
        false
    }

    fn completion_score(&self) -> i64 {
        let mut char_stack: Vec<&str> = Vec::new();
        for (idx, symbol) in self.letters.iter().enumerate() {
            if !CLOSURES.contains_key(symbol) {
                char_stack.push(symbol);
            } else {
                char_stack.pop();
            }
        }

        let mut completion_stack: Vec<&str> = Vec::new();
        char_stack.into_iter().rev().for_each(|x| {
            completion_stack.push(OPENERS.get(x).unwrap());
        });
        completion_stack
            .iter()
            .fold(0, |acc, x| 5 * acc + COMPLETION_SCORES.get(x).unwrap())
    }
}

fn part2(chunks: Vec<Chunk>) -> i64 {
    let mut scores = chunks
        .iter()
        .filter(|x| !x.is_corrupt())
        .map(|x| x.completion_score())
        .collect::<Vec<i64>>();
    scores.sort();
    let length: f64 = scores.len() as f64;
    
    let guess: usize = ((length/2.0)-0.5).round() as usize;
    scores[guess]
}

fn main() {
    let input = include_str!("input.txt");
    let chunks: Vec<Chunk> = input.lines().map(|x| Chunk::from_str(x)).collect();
    println!(
        "{:?}",
        chunks.iter().map(|x| x.illegal_char_value()).sum::<i64>()
    );
    println!("{:?}",part2(chunks));
}
