use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let numbers: Vec<i32> = input.split(",").map(|n| n.parse().unwrap()).collect();
        Point {
            x: numbers[0],
            y: numbers[1],
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    position: i32,
}

impl Line {
    fn from_str(input: &str) -> Line {
        let sides: Vec<&str> = input.split("->").collect();
        Line {
            start: Point::from_str(sides[0].trim()),
            end: Point::from_str(sides[1].trim()),
            position: 0,
        }
    }
    fn has_slope(&self) -> bool {
        !(self.is_vertical() || self.is_horizontal())
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn points_length(&self) -> i32 {
        if self.is_horizontal() {
            (self.end.x - self.start.x).abs()
        } else {
            (self.end.y - self.start.y).abs()
        }
    }
    fn slope(&self) -> Option<i32> {
        if self.end.x == self.start.x {
            return None;
        }
        Some((self.end.y - self.start.y) / (self.end.x - self.start.x))
    }
    fn sign(&self) -> i32 {
        if self.is_vertical() {
            return (self.end.y - self.start.y) / self.points_length();
        } else {
            return (self.end.x - self.start.x) / self.points_length();
        }
    }
}

impl Iterator for Line {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.position > self.points_length() {
            return None;
        }
        if self.is_vertical() {
            let result = Some(Point {
                x: self.start.x,
                y: self.start.y + self.position * self.sign(),
            });
            self.position += 1;
            return result;
        } else {
            let result = Some(Point {
                x: self.start.x + self.position * self.sign(),
                y: self.start.y + (self.slope().unwrap() * self.position * self.sign()),
            });
            self.position += 1;
            return result;
        }
    }
}

fn main() {
    let string = include_str!("input.txt");
    let mut lines: Vec<Line> = string.lines().map(|l| Line::from_str(l)).collect();
    let mut no_slope_lines: Vec<Line> = string
        .lines()
        .map(|l| Line::from_str(l))
        .filter(|x| !x.has_slope())
        .collect();
    println!("{:?}", overlap(&mut no_slope_lines));
    println!("{:?}", overlap(&mut lines));
}

fn overlap(lines: &mut Vec<Line>) -> i32 {
    let mut places: HashMap<Point, i32> = HashMap::new();
    for line in lines {
        // println!("{:?}", line);
        for place in line {
            // println!("{:?}", place);
            *places.entry(place).or_insert(0) += 1;
        }
    }
    let mut ct = 0;

    for (key, value) in places {
        if value >= 2 {
            // println!("{:?}", key);
            ct += 1;
        }
    }
    ct
}
