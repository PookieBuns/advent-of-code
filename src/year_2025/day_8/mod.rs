use std::path::Path;

use crate::Answer;

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            components: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        self.components -= 1;
        true
    }

    fn sets(&mut self) -> Vec<Vec<usize>> {
        let root_to_members =
            (0..self.parent.len()).fold(std::collections::HashMap::new(), |mut acc, i| {
                let root = self.find(i);
                acc.entry(root).or_insert_with(Vec::new).push(i);
                acc
            });
        root_to_members.into_values().collect()
    }
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn distance(&self, other: &Coord) -> f64 {
        ((self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn get_coords_sortest_by_lowest_distance(coords: &[Coord]) -> Vec<(usize, usize)> {
    let mut distances = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let dist = coords[i].distance(&coords[j]);
            distances.push((i, j, dist));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    distances.into_iter().map(|(i, j, _)| (i, j)).collect()
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let to_connect = 1000;
    let coords = input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
            Coord {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect::<Vec<Coord>>();
    let pairs = get_coords_sortest_by_lowest_distance(&coords);
    let mut dsu = Dsu::new(coords.len());
    for (i, j) in pairs[..to_connect].iter() {
        dsu.union(*i, *j);
    }
    let mut groups = dsu.sets();
    groups.sort_by_key(|group| std::cmp::Reverse(group.len()));
    groups
        .iter()
        .take(3)
        .map(|group| group.len())
        .product::<usize>()
        .into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let coords = input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
            Coord {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect::<Vec<Coord>>();
    let pairs = get_coords_sortest_by_lowest_distance(&coords);
    let mut dsu = Dsu::new(coords.len());
    for (i, j) in pairs {
        dsu.union(i, j);
        if dsu.components == 1 {
            return (coords[i].x * coords[j].x).into();
        }
    }
    0.into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1_start_time = std::time::Instant::now();
    let part_1 = part_1(&input);
    let part_1_elapsed_time = part_1_start_time.elapsed();
    println!("Done with part 1 in {:?}", part_1_elapsed_time);
    let part_2_start_time = std::time::Instant::now();
    let part_2 = part_2(&input);
    let part_2_elapsed_time = part_2_start_time.elapsed();
    println!("Done with part 2 in {:?}", part_2_elapsed_time);
    Answer::from_parts(part_1, part_2)
}
