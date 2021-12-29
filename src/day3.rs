use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Direction>, Vec<Direction>) {
    input.lines().map(|l| {
        l.split(",").map(|step| {
            let d = step[1..].parse::<isize>().unwrap();

            match &step[0..1] {
                "U" => Direction::Up(d),
                "D" => Direction::Down(d),
                "L" => Direction::Left(d),
                "R" => Direction::Right(d),
                &_ => panic!("panic"),

            }
        }).collect_vec()
    }).collect_tuple().unwrap()
}


fn positions(steps: &Vec<Direction>) -> HashSet<(isize, isize)> {
    steps.iter().scan((0isize, 0isize), |(x, y), dir| {
        match dir {
            Direction::Up(d) => {
                let ystart = *y;
                *y += *d;
                Some((ystart + 1..=*y).map(|y| (*x, y)).collect_vec())
            },
            Direction::Down(d) => {
                let ystart = *y;
                *y -= *d;
                Some((*y..ystart).map(|y| (*x, y)).collect_vec())
            }
            Direction::Left(d) => {
                let xstart = *x;
                *x -= *d;
                Some((*x..xstart).map(|x| (x, *y)).collect_vec())
            }
            Direction::Right(d) => {
                let xstart = *x;
                *x += *d;
                Some((xstart + 1..=*x).map(|x| (x, *y)).collect_vec())
            }
        }
    }).flatten().collect()
}

fn manhatten_distance((x, y): &(isize, isize)) -> usize {
    (x.abs() + y.abs()).try_into().unwrap()
}


#[aoc(day3, part1)]
pub fn solve_part1(input: & (Vec<Direction>, Vec<Direction>)) -> usize {
    let (a, b) = input;
    positions(a).intersection(&positions(b)).map(manhatten_distance).min().unwrap()
}


fn weighted_positions(steps: &Vec<Direction>) -> HashMap<(isize, isize), usize> {
    let mut displacement = 0;
    let (mut x, mut y) = (0isize, 0isize);

    let mut map = HashMap::new();

    for step in steps {
        match step {
            Direction::Up(d) => {
                for (i, pos) in (y + 1..=y + d).map(|y| (x, y)).enumerate() {
                    if !map.contains_key(&pos) {
                        map.insert(pos, i + displacement + 1);
                    }
                }
                y += d;
                displacement += *d as usize;
            },
            Direction::Down(d) => {
                for (i, pos) in (y - d..y).rev().map(|y| (x, y)).enumerate() {
                    if !map.contains_key(&pos) {
                        map.insert(pos, i + displacement + 1);
                    }
                }
                y -= d;
                displacement += *d as usize;
            },
            Direction::Left(d) => {
                for (i, pos) in (x - d..x).rev().map(|x| (x, y)).enumerate() {
                    if !map.contains_key(&pos) {
                        map.insert(pos, i + displacement + 1);
                    }
                }
                x -= d;
                displacement += *d as usize;
            },
            Direction::Right(d) => {
                for (i, pos) in (x + 1..=x + d).map(|x| (x, y)).enumerate() {
                    if !map.contains_key(&pos) {
                        map.insert(pos, i + displacement + 1);
                    }
                }
                x += d;
                displacement += *d as usize;
            },
        }
    };
    map
}

#[aoc(day3, part2)]
pub fn solve_part2(input: & (Vec<Direction>, Vec<Direction>)) -> Option<usize> {
    let (a, b) = input;

    let (a, b) = (weighted_positions(a), weighted_positions(b));

    a.iter().filter_map(|(pos, aweight)| b.get(pos).map(|bweight| aweight + bweight)).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions() {
        assert_eq!(positions(&vec!(Direction::Up(2))), HashSet::from([(0, 1), (0, 2)]));
        assert_eq!(positions(&vec!(Direction::Down(2))), HashSet::from([(0, -1), (0, -2)]));
        assert_eq!(positions(&vec!(Direction::Left(2))), HashSet::from([(-1, 0), (-2, 0)]));
        assert_eq!(positions(&vec!(Direction::Right(2))), HashSet::from([(1, 0), (2, 0)]));
    }

    #[test]
    fn test_weighted_positions() {
        assert_eq!(weighted_positions(&vec!(Direction::Up(2))), HashMap::from([((0, 1), 1), ((0, 2), 2)]));
        assert_eq!(weighted_positions(&vec!(Direction::Down(2))), HashMap::from([((0, -1), 1), ((0, -2), 2)]));
        assert_eq!(weighted_positions(&vec!(Direction::Left(2))), HashMap::from([((-1, 0), 1), ((-2, 0), 2)]));
        assert_eq!(weighted_positions(&vec!(Direction::Right(2))), HashMap::from([((1, 0), 1), ((2, 0), 2)]));
    }

    #[test]
    fn test_manhatten_distance() {
        assert_eq!(manhatten_distance(&(0, 0)), 0);
        assert_eq!(manhatten_distance(&(-1, -1)), 2);
        assert_eq!(manhatten_distance(&(-10, 20)), 30);
    }

    #[test]
    fn test_intersections() {
        let a = positions(&vec!(Direction::Right(8), Direction::Up(5), Direction::Left(5), Direction::Down(3)));
        let b = positions(&vec!(Direction::Up(7), Direction::Right(6), Direction::Down(4), Direction::Left(4)));
        assert_eq!(a.intersection(&b).cloned().collect::<HashSet<(isize, isize)>>(), HashSet::from([(3, 3), (6, 5)]));
    }

    #[test]
    fn test_input_generator() {
        let a = vec!(Direction::Right(8), Direction::Up(5), Direction::Left(5), Direction::Down(3));
        let b = vec!(Direction::Up(7), Direction::Right(6), Direction::Down(4), Direction::Left(4));
        assert_eq!((a, b), input_generator("R8,U5,L5,D3\nU7,R6,D4,L4"))
    }

    #[test]
    fn example0() {
        let a = vec!(Direction::Right(8), Direction::Up(5), Direction::Left(5), Direction::Down(3));
        let b = vec!(Direction::Up(7), Direction::Right(6), Direction::Down(4), Direction::Left(4));
        assert_eq!(solve_part1(&(a, b)), 6);
    }

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")), 159);
        assert_eq!(solve_part2(&input_generator("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")), Some(610));
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part1(&input_generator("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")), 135);
        assert_eq!(solve_part2(&input_generator("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")), Some(410));
    }
}
