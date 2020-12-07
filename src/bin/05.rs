fn seat(code: &str) -> (usize, usize) {
    let (mut row, mut col) = (0, 0);
    for c in code.chars() {
        match c {
            'F' => { row <<= 1; },
            'B' => { row <<= 1; row += 1; },
            'L' => { col <<= 1; },
            'R' => { col <<= 1; col += 1; },
            _ => panic!(),
        }
    }
    (row, col)
}

fn solve(input: &str) -> (usize, usize) {
    let mut seat_ids: Vec<_> = input
        .lines()
        .map(seat)
        .map(|(row, col)| row*8+col)
        .collect();
    
    seat_ids.sort();
    let min_id = seat_ids[0];
    let max_id = seat_ids[seat_ids.len()-1];
    let missing_id = seat_ids
        .iter()
        .enumerate()
        .find_map(|(pos, id)| if *id > min_id+pos { Some(min_id+pos) } else { None })
        .unwrap();

    (max_id, missing_id)
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(seat("FBFBBFFRLR"), (44, 5));
        assert_eq!(seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat("BBFFBBFRLL"), (102, 4));
    }
}
