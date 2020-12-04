use aoc_runner_derive::{aoc, aoc_generator};

struct Solution {
    field: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Pos(usize, usize);

struct Slope(usize, usize);

impl Solution {
    fn advance(&self, current: &Pos, slope: &Slope) -> Pos {
        Pos(current.0 + slope.0, (current.1 + slope.1) % self.width)
    }

    fn is_tree(&self, pos: &Pos) -> bool {
        self.field[pos.0 * (self.width + 1) + pos.1] == b'#'
    }

    fn count_trees(&self, slope: &Slope) -> u64 {
        let mut pos = Pos(0, 0);
        let mut count = 0;

        while pos.0 < self.height {
            pos = self.advance(&pos, slope);
            if self.is_tree(&pos) {
                count += 1;
            }
        }

        count
    }
}

#[aoc_generator(day3)]
fn parse(input: &[u8]) -> Solution {
    let width = input.iter().position(|&byte| byte == b'\n').unwrap();
    Solution {
        field: input.to_vec(),
        width,
        height: input.len() / (width + 1),
    }
}

#[aoc(day3, part1)]
fn part1(solution: &Solution) -> u64 {
    solution.count_trees(&Slope(1, 3))
}

#[aoc(day3, part2)]
fn part2(solution: &Solution) -> u64 {
    let slopes = vec![Slope(1, 1), Slope(1, 3), Slope(1, 5), Slope(1, 7), Slope(2, 1)];

    slopes.iter().map(|slope| solution.count_trees(slope)).product()
}
