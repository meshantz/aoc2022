use std::{collections::HashMap, fs, marker::PhantomData, ops};

const ROCK: char = '#';
const SAND: char = 'o';

const DOWN: Point = Point { x: 0, y: 1 };
const DOWN_LEFT: Point = Point { x: -1, y: 1 };
const DOWN_RIGHT: Point = Point { x: 1, y: 1 };
const SAND_START: Point = Point { x: 500, y: 0 };

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct LineIterator {
    start: Point,
    end: Point,
    curr: Option<Point>,
    dir: Point,
}

impl Point {
    fn unit_vector(from: Point, to: Point) -> Point {
        let dir_x = to.x - from.x;
        let dir_y = to.y - from.y;
        Point {
            x: dir_x.signum(),
            y: dir_y.signum(),
        }
    }
}

impl ops::Add<Self> for Point {
    type Output = Point;
    fn add(self, _rhs: Self) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl LineIterator {
    fn new(start: Point, end: Point) -> LineIterator {
        LineIterator {
            start,
            end,
            curr: None,
            dir: Point::unit_vector(start, end),
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Some(p) => {
                if p.x == self.end.x && p.y == self.end.y {
                    self.curr = None;
                } else {
                    self.curr = Some(p + self.dir);
                }
            }
            None => {
                self.curr = Some(self.start);
            }
        }
        self.curr
    }
}

fn make_map(raw: &str, with_floor: bool) -> (HashMap<Point, char>, i32) {
    let mut map = HashMap::new();
    let mut bottom = 0;
    for line in raw.lines() {
        let mut point_string = Vec::new();
        let better_line = line.replace(" -> ", ",");
        let coordinates = better_line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for i in (0..coordinates.len()).step_by(2) {
            point_string.push(Point {
                x: coordinates[i],
                y: coordinates[i + 1],
            });
            bottom = bottom.max(coordinates[i + 1]);
        }

        // println!("{:?}", point_string);
        for i in 0..point_string.len() - 1 {
            let mut iter = LineIterator::new(point_string[i], point_string[i + 1]);
            while let Some(p) = iter.next() {
                // println!("{:?}", p);
                map.insert(p, ROCK);
            }
        }
    }

    let mut point_string = Vec::new();
    if with_floor {
        // println!("Adding floor at y = {:?}", bottom + 2);
        point_string.push(Point {
            x: -10000,
            y: bottom + 2,
        });
        point_string.push(Point {
            x: 10000,
            y: bottom + 2,
        });
        for i in 0..point_string.len() - 1 {
            let mut iter = LineIterator::new(point_string[i], point_string[i + 1]);
            while let Some(p) = iter.next() {
                // println!("{:?}", p);
                map.insert(p, ROCK);
            }
        }
        bottom += 2
    }

    (map, bottom)
}

fn draw(map: &HashMap<Point, char>, extents: (Point, Point)) {
    for y in extents.0.y..extents.1.y {
        for x in extents.0.x..extents.1.x {
            match map.get(&Point { x, y }) {
                Some(v) => print!("{}", v),
                None => print!("."),
            };
        }
        println!();
    }
}

fn drop_sand(raw: &str, with_floor: bool) -> i32 {
    let (mut map, bottom) = make_map(&raw, with_floor);
    // draw(&map, (Point { x: 492, y: 0 }, Point { x: 492 + 15, y: 15 }));

    let mut sand = SAND_START;
    while sand.y <= bottom {
        if !map.contains_key(&(sand + DOWN)) {
            // print!("down ");
            sand = sand + DOWN;
        } else if !map.contains_key(&(sand + DOWN_LEFT)) {
            // print!("down-left ");
            sand = sand + DOWN_LEFT;
        } else if !map.contains_key(&(sand + DOWN_RIGHT)) {
            // print!("down-right ");
            sand = sand + DOWN_RIGHT;
        } else {
            // comes to rest
            // println!("rest at {:?}", sand);
            map.insert(sand, SAND);
            if sand.y == 0 {
                break;
            }
            sand = SAND_START;
        }
    }
    // println!("\n{:?}", sand);
    // draw(&map, (Point { x: 494, y: 0 }, Point { x: 494 + 10, y: 10 }));

    let sand_count: i32 = map
        .values()
        .map(|t| match t {
            &SAND => 1,
            _ => 0,
        })
        .sum();

    sand_count
}

pub fn solve() {
    let raw = fs::read_to_string("data/day14.example").unwrap();
    println!("Part 1: {}", drop_sand(&raw, false));
    println!("Part 2: {}", drop_sand(&raw, true));

    let raw = fs::read_to_string("data/day14.txt").unwrap();
    println!("Part 1: {}", drop_sand(&raw, false));
    println!("Part 2: {}", drop_sand(&raw, true));
}
