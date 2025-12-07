use std::path::Path;

use crate::Answer;

fn access(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    directions
        .iter()
        .filter(|(dr, dc)| {
            let nr = row.wrapping_add_signed(*dr);
            let nc = col.wrapping_add_signed(*dc);
            if nr >= grid.len() {
                return false;
            }
            if nc >= grid[0].len() {
                return false;
            }
            grid[nr][nc] == '@'
        })
        .count()
        < 4
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let rows = grid.len();
    let cols = grid[0].len();
    (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == '@' && access(&grid, r, c))
        .count()
        .into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let rows = grid.len();
    let cols = grid[0].len();
    loop {
        let coords = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c)));
        let to_remove = coords
            .filter(|&(r, c)| grid[r][c] == '@' && access(&grid, r, c))
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            break;
        }
        to_remove.iter().for_each(|&(r, c)| grid[r][c] = 'x');
    }
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == 'x')
        .count()
        .into()
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
