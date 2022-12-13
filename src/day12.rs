use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VectorHeuristic {
    from: Vector2,
    to: Vector2,
    distance_to_end: u32,
}

impl Vector2 {
    fn add(&self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn distance(&self, other: Vector2) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    fn neighbours(&self, width: i32, height: i32) -> [Option<Vector2>; 4] {
        let mut neighbours = [None; 4];
        for (i, dir) in DIRECTIONS.iter().enumerate() {
            let potential = self.add(*dir);
            if potential.x < 0 || potential.x >= width || potential.y < 0 || potential.y >= height {
                continue;
            }
            neighbours[i] = Some(potential);
        }
        neighbours
    }
}

const UP: Vector2 = Vector2 { x: 0, y: -1 };
const DOWN: Vector2 = Vector2 { x: 0, y: 1 };
const LEFT: Vector2 = Vector2 { x: -1, y: 0 };
const RIGHT: Vector2 = Vector2 { x: 1, y: 0 };
const DIRECTIONS: [Vector2; 4] = [UP, RIGHT, DOWN, LEFT];

fn load<const N: usize>(raw: &str, width: i32) -> ([i32; N], Vector2, Vector2) {
    let mut height_map = [0i32; N];
    let mut start = Vector2::default();
    let mut end = Vector2::default();

    for (j, line) in raw.lines().enumerate() {
        for (i, col) in line.chars().enumerate() {
            let height = match col {
                'S' => {
                    start.x = i as i32;
                    start.y = j as i32;
                    'a' as i32
                }
                'E' => {
                    end.x = i as i32;
                    end.y = j as i32;
                    'z' as i32
                }
                val => val as i32,
            };
            height_map[i + j * width as usize] = height;
        }
    }

    (height_map, start, end)
}

fn load_all<const N: usize>(raw: &str, width: i32) -> ([i32; N], Vec<Vector2>, Vector2) {
    let mut height_map = [0i32; N];
    let mut start = Vec::new();
    let mut end = Vector2::default();

    for (j, line) in raw.lines().enumerate() {
        for (i, col) in line.chars().enumerate() {
            let height = match col {
                'S' => {
                    start.push(Vector2 {
                        x: i as i32,
                        y: j as i32,
                    });
                    'a' as i32
                }
                'E' => {
                    end.x = i as i32;
                    end.y = j as i32;
                    'z' as i32
                }
                'a' => {
                    start.push(Vector2 {
                        x: i as i32,
                        y: j as i32,
                    });
                    'a' as i32
                }
                val => val as i32,
            };
            height_map[i + j * width as usize] = height;
        }
    }

    (height_map, start, end)
}

fn reachable(from: i32, to: i32) -> bool {
    to <= from + 1
}

fn shortest_path<const N: usize>(
    height_map: [i32; N],
    start: Vector2,
    end: Vector2,
    width: i32,
    height: i32,
) -> Option<i32> {
    let mut distances = HashMap::new();
    let mut adjacent = BinaryHeap::new();
    distances.insert(start, 0);
    for neighbour in start.neighbours(width, height) {
        match neighbour {
            Some(n) => {
                let n_pos = n.x as usize + width as usize * n.y as usize;
                let s_pos = start.x as usize + width as usize * start.y as usize;
                if reachable(height_map[s_pos], height_map[n_pos]) {
                    adjacent.push(VectorHeuristic {
                        to: n,
                        from: start,
                        distance_to_end: n.distance(end),
                    });
                }
            }
            _ => (),
        }
    }
    while let Some(consider) = adjacent.pop() {
        if distances.contains_key(&consider.to) {
            let new_consider = *distances.get(&consider.from).unwrap() + 1;
            if distances.get(&consider.to).unwrap() > &new_consider {
                distances.insert(consider.to, new_consider);
            } else {
                continue;
            }
        } else {
            distances.insert(consider.to, distances.get(&consider.from).unwrap() + 1);
        }
        // println!(
        //     "Added {:?}: {} (priority: {})",
        //     consider.to,
        //     distances.get(&consider.to).unwrap(),
        //     consider.distance_to_end,
        // );

        for neighbour in consider.to.neighbours(width, height) {
            match neighbour {
                Some(n) => {
                    let n_pos = n.x as usize + width as usize * n.y as usize;
                    let s_pos = consider.to.x as usize + width as usize * consider.to.y as usize;
                    let next_distance = *distances.get(&consider.to).unwrap() + 1;
                    if reachable(height_map[s_pos], height_map[n_pos])
                        && (!distances.contains_key(&n)
                            || distances.get(&n).unwrap() > &next_distance)
                    {
                        adjacent.push(VectorHeuristic {
                            to: n,
                            from: consider.to,
                            distance_to_end: n.distance(end),
                        });
                    }
                }
                _ => (),
            }
        }
    }
    let shortest = *distances.get(&end)?;
    Some(shortest)
}

fn part1<const N: usize>(raw: &str, width: i32, height: i32) {
    let (height_map, start, end) = load::<N>(&raw, width);

    let shortest = shortest_path::<N>(height_map, start, end, width, height);

    println!("Part 1: {}", shortest.unwrap());
}

fn part2<const N: usize>(raw: &str, width: i32, height: i32) {
    let (height_map, start, end) = load_all::<N>(&raw, width);
    let mut path_lengths = Vec::new();

    for start_vector in start {
        let shortest = shortest_path::<N>(height_map, start_vector, end, width, height);
        path_lengths.push(shortest);
    }

    println!(
        "Part 2: {:?}",
        path_lengths
            .iter()
            .map(|a| match a {
                Some(v) => v,
                None => &100000,
            })
            .fold(100000, |a, b| a.min(*b))
    );
}

pub fn solve() {
    let raw = fs::read_to_string("data/day12.example").unwrap(); // 8 x 5
    part1::<{ 8 * 5 }>(&raw, 8, 5);
    part2::<{ 8 * 5 }>(&raw, 8, 5);

    let raw = fs::read_to_string("data/day12.txt").unwrap(); // 144 x 41
    part1::<{ 144 * 41 }>(&raw, 144, 41);
    part2::<{ 144 * 41 }>(&raw, 144, 41);
}
