use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;

#[derive(Debug)]
struct AddressRange {
    addr: u64,
    mask: u64, // follows the IP netmask convention: ones on non-floating bits
}

impl AddressRange {
    fn intersect(&self, other: &AddressRange) -> Option<AddressRange> {
        if (self.addr ^ other.addr) & (self.mask & other.mask) == 0 {
            Some(AddressRange{ addr: self.addr | other.addr, mask: self.mask | other.mask })
        } else {
            None
        }
    }

    fn len(&self) -> u64 {
        1 << (36 - self.mask.count_ones())
    }
}

impl Display for AddressRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let addr_str = format!("{:036b}", self.addr);
        let mask_str = format!("{:036b}", self.mask);
        for (a, m) in addr_str.chars().zip(mask_str.chars()) {
            if m == '1' { f.write_char(a)?; }
            else { f.write_char('_')?; }
        }
        Ok(())
    }
}

fn solve(input: &str) -> (i64, i64) {
    // memory model for v1 decoder
    let mut memory = HashMap::<u64, i64>::new();

    // memory model for v2 decoder: a list of address ranges
    // with an associated value
    let mut memory_ranges = Vec::<(AddressRange, i64)>::new();

    let mut mask_ones = 0;
    let mut mask_zeros = 0;

    for line in input.lines() {
        let mut parts = line.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        if left == "mask" {
            mask_ones = 0;
            mask_zeros = 0;
            for (i, c) in right.chars().rev().enumerate() {
                match c {
                    '0' => { mask_zeros |= 1 << i },
                    '1' => { mask_ones |= 1 << i },
                    _ => {},
                }
            }
        } else {
            let addr = left[4..left.len()-1].parse::<u64>().unwrap();
            let value = right.parse::<u64>().unwrap();

            // v1 decoding: store the masked value in memory
            let masked_value = ((value | mask_ones) & !mask_zeros) as i64;
            //println!("mem[{}] = {}", addr, masked_value);
            memory.insert(addr, masked_value);

            // v2 decoding: store the address range with the associated value
            let addrrange = AddressRange{
                addr: (addr | mask_ones) & (mask_zeros | mask_ones),
                mask: (mask_zeros | mask_ones),
            };

            // Before storing the address range, check if it overlaps with other ranges
            // and compensate by adding a range with the opposite of the value for the
            // overwritten part.
            let mut memory_overlaps: Vec<_> = memory_ranges
                .iter()
                .filter_map(|(a, v)| {
                    match addrrange.intersect(a) {
                        Some(o) => Some((o, -v)),
                        None => None,
                    }
                })
                //.inspect(|(a, v)| println!("del: mem[{}] -> {}", a, v))
                .collect();
            memory_ranges.append(&mut memory_overlaps);

            //println!("add: mem[{}] -> {}", addrrange, value as i64);
            memory_ranges.push((addrrange, value as i64));
        }
    }

    (memory.values().sum(), memory_ranges.iter().map(|(a, v)| a.len() as i64 * v).sum())
}

fn main() {
    let input = std::fs::read_to_string("input/14.txt").unwrap();
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
        assert_eq!(solve("\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0").0, 165);
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1").1, 208);
    }
}
