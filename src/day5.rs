use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Seat> {
    fn decode(c: char) -> u32 {
        match c {
            'F' => 0,
            'B' => 1,
            'R' => 1,
            'L' => 0,
            _ => unreachable!(),
        }
    }
    fn parse_seat(input: &str) -> Seat {
        let row = input[0..7]
            .chars()
            .rev()
            .enumerate()
            .map(|(idx, ch)| (1 << idx) * decode(ch))
            .sum();

        let column = input[7..10]
            .chars()
            .rev()
            .enumerate()
            .map(|(idx, ch)| (1 << idx) * decode(ch))
            .sum();

        Seat { row, column }
    }

    input.lines().map(parse_seat).collect()
}

#[aoc(day5, part1)]
fn part1(seats: &[Seat]) -> Option<u32> {
    seats.iter().map(|seat| seat.id()).max()
}

#[aoc(day5, part2)]
fn part2(seats: &[Seat]) -> Option<u32> {
    let mut ids = seats.iter().map(|seat| seat.id()).collect::<Vec<_>>();
    ids.sort();

    ids.windows(2)
        .find(|&window| window[0] + 2 == window[1])
        .map(|window| window[0] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_examples() {
        let input = "FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
        let seats = parse(input);

        let expected = vec![
            Seat { row: 44, column: 5 },
            Seat { row: 70, column: 7 },
            Seat { row: 14, column: 7 },
            Seat {
                row: 102,
                column: 4,
            },
        ];
        assert_eq!(seats, expected);
    }
}
