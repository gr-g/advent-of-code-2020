#[derive(Debug)]
struct TicketField {
    name: String,
    ranges: Vec<(u64, u64)>,
    position: Option<usize>,
}

impl TicketField {
    fn create_from(s: &str) -> TicketField {
        let (name, ranges) = s.split_once(": ").unwrap();
        let name = name.to_string();
        let ranges = ranges
            .split(" or ")
            .map(|range| range.split_once("-").unwrap())
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .collect();

        TicketField { name, ranges, position: None /* unknown */ }
    }

    fn allows(&self, value: u64) -> bool {
        self.ranges.iter().any(|(min, max)| value >= *min && value <= *max)
    }
}

fn read_input(input: &str) -> (Vec<TicketField>, Vec<u64>, Vec<Vec<u64>>) {
    let mut parts = input.split("\n\n");

    let fields = parts.next().unwrap().lines()
        .map(|line| TicketField::create_from(line))
        .collect();

    let my_ticket = parts.next().unwrap().lines()
        .nth(1).unwrap().split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let nearby_tickets = parts.next().unwrap().lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (fields, my_ticket, nearby_tickets)
}

fn filter_tickets(fields: &[TicketField], tickets: &[Vec<u64>]) -> (Vec<Vec<u64>>, u64) {
    let mut valid_tickets = Vec::new();
    let mut error_rate = 0;

    for ticket in tickets {
        let mut valid = true;
        for v in ticket {
            if fields.iter().all(|f| !f.allows(*v)) {
                valid = false;
                error_rate += *v;
            }
        }
        if valid {
            valid_tickets.push(ticket.clone());
        }
    }

    (valid_tickets, error_rate)
}

fn assign_fields(fields: &mut [TicketField], tickets: &[Vec<u64>]) {
    // Collect the possible positions for each field.
    let mut possible_positions = Vec::new();
    for field in fields.iter() {
        let mut possible_positions_f = Vec::new();
        for i in 0..fields.len() {
            if tickets.iter().all(|t| field.allows(t[i])) {
                possible_positions_f.push(i);
            }
        }

        //println!("field {} can be in position: {:?}", field.name, possible_positions_f);
        possible_positions.push(possible_positions_f);
    }

    // Assign a position to each field
    while let Some(f) = possible_positions.iter().position(|list| list.len() == 1) {
        // field f can only be in one position
        let assigned_position = possible_positions[f][0];

        //println!("field {} is in position {}", fields[f].name, assigned_position);
        fields[f].position = Some(assigned_position);

        // remove this position from all lists
        for list in &mut possible_positions {
            if let Some(i) = list.iter().position(|i| *i == assigned_position) {
                list.remove(i);
            }
        }
    }

    if !possible_positions.iter().all(|list| list.is_empty()) {
        panic!("the problem does not have a unique solution");
    }

    // Sort fields by position.
    fields.sort_by_key(|f| f.position.unwrap());
}

fn solve(input: &str) -> (u64, u64) {
    let (mut fields, my_ticket, nearby_tickets) = read_input(input);

    let (valid_tickets, error_rate) = filter_tickets(&fields, &nearby_tickets);

    assign_fields(&mut fields, &valid_tickets);

    let departure_product = fields
        .iter()
        .filter(|f| f.name.starts_with("departure"))
        .map(|f| my_ticket[f.position.unwrap()])
        .product();

    (error_rate, departure_product)
}

fn main() {
    let input = std::fs::read_to_string("input/16.txt").unwrap();
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
        let (fields, _, nearby_tickets) = read_input("\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");
        let (_, error_rate) = filter_tickets(&fields, &nearby_tickets);
        assert_eq!(error_rate, 71);
    }

    #[test]
    fn example02() {
        let (mut fields, _, tickets) = read_input("\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9");
        assign_fields(&mut fields, &tickets);

        println!("{:?}", fields);
        assert_eq!(fields[0].name, "row");
        assert_eq!(fields[1].name, "class");
        assert_eq!(fields[2].name, "seat");
    }
}
