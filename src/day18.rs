use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cube3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Cube3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn parse_cubes(raw: &str) -> HashSet<Cube3> {
    let cubes: Vec<Cube3> = raw
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|p| p.parse::<i32>().unwrap())
                .collect();
            Cube3 {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();
    HashSet::from_iter(cubes.into_iter())
}

fn part(num: i32, cubes: &HashSet<Cube3>) {
    let mut free_faces = HashMap::new();

    let directions = [
        Cube3 { x: 1, y: 0, z: 0 },
        Cube3 { x: -1, y: 0, z: 0 },
        Cube3 { x: 0, y: 1, z: 0 },
        Cube3 { x: 0, y: -1, z: 0 },
        Cube3 { x: 0, y: 0, z: 1 },
        Cube3 { x: 0, y: 0, z: -1 },
    ];

    for cube in cubes {
        for dir in &directions {
            let consider = *cube + *dir;
            let mut count = free_faces.entry(*cube).or_insert(0);
            *count += 1 - cubes.contains(&consider) as i32;
        }
    }

    let sum: i32 = free_faces.values().sum();
    println!("Part {}: {}", num, sum);
}

fn extents(cubes: &HashSet<Cube3>) -> ((i32, i32, i32), (i32, i32, i32)) {
    let mut mins = (1000, 1000, 1000);
    let mut maxs = (0, 0, 0);

    for cube in cubes {
        mins.0 = mins.0.min(cube.x);
        mins.1 = mins.1.min(cube.y);
        mins.2 = mins.2.min(cube.z);
        maxs.0 = maxs.0.max(cube.x);
        maxs.1 = maxs.1.max(cube.y);
        maxs.2 = maxs.2.max(cube.z);
    }

    // add a border
    mins.0 -= 1;
    mins.1 -= 1;
    mins.2 -= 1;
    maxs.0 += 1;
    maxs.1 += 1;
    maxs.2 += 1;

    (mins, maxs)
}

fn air_in_extents(
    cubes: &HashSet<Cube3>,
    extents_: ((i32, i32, i32), (i32, i32, i32)),
) -> HashSet<Cube3> {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = extents_;
    let mut air = HashSet::new();

    for k in min_z..max_z + 1 {
        for j in min_y..max_y + 1 {
            for i in min_x..max_x + 1 {
                let check_air = Cube3 { x: i, y: j, z: k };
                if !cubes.contains(&check_air) {
                    air.insert(check_air);
                }
            }
        }
    }

    air
}

fn is_edge(extents_: ((i32, i32, i32), (i32, i32, i32)), pos: Cube3) -> bool {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = extents_;
    pos.x == min_x
        || pos.x == max_x
        || pos.y == min_y
        || pos.y == max_y
        || pos.z == min_z
        || pos.z == max_z
}

fn external_internal(cubes: &HashSet<Cube3>) -> (HashSet<Cube3>, HashSet<Cube3>) {
    let extents_ = extents(cubes);
    let air = air_in_extents(cubes, extents_);
    let mut available = air.clone();
    let mut external = HashSet::new();
    let mut internal = HashSet::new();

    let directions = [
        Cube3 { x: 1, y: 0, z: 0 },
        Cube3 { x: -1, y: 0, z: 0 },
        Cube3 { x: 0, y: 1, z: 0 },
        Cube3 { x: 0, y: -1, z: 0 },
        Cube3 { x: 0, y: 0, z: 1 },
        Cube3 { x: 0, y: 0, z: -1 },
    ];

    // println!("Air is {:?}", air);
    for air_cube in air {
        if !available.contains(&air_cube) {
            continue;
        }

        let mut new_group = vec![air_cube];
        let mut is_external = is_edge(extents_, air_cube);
        let mut frontier: VecDeque<Cube3> = directions
            .map(|d| air_cube + d)
            .into_iter()
            .filter(|n| available.contains(n))
            .collect();

        for claimed in &frontier {
            available.remove(claimed);
        }

        while let Some(f) = frontier.pop_front() {
            new_group.push(f);
            is_external = is_external || is_edge(extents_, f);
            let mut neighbours: Vec<Cube3> = directions
                .map(|d| f + d)
                .into_iter()
                .filter(|n| available.contains(n))
                .collect();
            for claimed in neighbours {
                available.remove(&claimed);
                frontier.push_back(claimed);
            }
        }

        while let Some(f) = new_group.pop() {
            if is_external {
                external.insert(f);
            } else {
                internal.insert(f);
            }
        }
    }

    (external, internal)
}

fn part2(cubes: &HashSet<Cube3>) {
    let (e, i) = external_internal(cubes);
    let mut all = cubes.clone();
    for a in i {
        all.insert(a);
    }
    part(2, &all);
}

pub fn solve() {
    let raw = fs::read_to_string("data/day18.example").unwrap();
    let cubes = parse_cubes(&raw);
    part(1, &cubes);
    part2(&cubes);

    let raw = fs::read_to_string("data/day18.txt").unwrap();
    let cubes = parse_cubes(&raw);
    part(1, &cubes);
    part2(&cubes);
}
