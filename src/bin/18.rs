fn tokenize(s: &str) -> Vec<char> {
    s.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect()
}

// Look for a top-level operator among the ones in 'ops' (left to right)
// and return the expression decomposed as (left, op, right).
fn split_on_op<'a>(expr: &'a [char], ops: &[char]) -> Option<(&'a [char], char, &'a [char])> {
    let mut p = expr.len() - 1;
    let mut nested = 0;
    while p > 0 {
        if nested == 0 && ops.contains(&expr[p]) {
            return Some((&expr[0..p], expr[p], &expr[p+1..]));
        }
        if expr[p] == '(' { nested -= 1; };
        if expr[p] == ')' { nested += 1; };
        p -= 1;
    }
    None
}

fn evaluate(expr: &[char]) -> Option<u64> {
    //println!("Evaluating: {:?}", expr);
    match expr.len() {
        0 => { None },
        1 => { expr[0].to_digit(10).map(|n| n as u64) },
        _ => {
            // Look for a top-level multiplication or addition.
            // Evaluate recursively the left and right parts and aggregate the results.
            match split_on_op(expr, &['*', '+']) {
                Some((left, '+', right)) => { Some(evaluate(left)? + evaluate(right)?) },
                Some((left, '*', right)) => { Some(evaluate(left)? * evaluate(right)?) },
                _ => {
                    // No top-level operation, evaluate the expression in parentheses.
                    if expr[0] == '(' && expr[expr.len()-1] == ')' {
                        evaluate(&expr[1..expr.len()-1])
                    } else {
                        None
                    }
                },
            }
        },
    }
}

fn evaluate_adv(expr: &[char]) -> Option<u64> {
    //println!("Evaluating: {:?}", expr);
    match expr.len() {
        0 => { None },
        1 => { expr[0].to_digit(10).map(|n| n as u64) },
        _ => {
            // Look for a top-level multiplication, then for an addition.
            // Evaluate recursively the left and right parts and aggregate the results.
            match split_on_op(expr, &['*']).or(split_on_op(expr, &['+'])) {
                Some((left, '+', right)) => { Some(evaluate_adv(left)? + evaluate_adv(right)?) },
                Some((left, '*', right)) => { Some(evaluate_adv(left)? * evaluate_adv(right)?) },
                _ => {
                    // No top-level operation, evaluate the expression in parentheses.
                    if expr[0] == '(' && expr[expr.len()-1] == ')' {
                        evaluate_adv(&expr[1..expr.len()-1])
                    } else {
                        None
                    }
                },
            }
        },
    }
}

fn solve(input: &str) -> (u64, u64) {
    let mut sum = 0;
    let mut sum_advanced = 0;
    for line in input.lines() {
        let expr = tokenize(line);
        sum += evaluate(&expr).expect(&format!("invalid expression: {}", line));
        sum_advanced += evaluate_adv(&expr).expect(&format!("invalid expression: {}", line));
    }

    (sum, sum_advanced)
}

fn main() {
    let input = std::fs::read_to_string("input/18.txt").unwrap();
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
        assert_eq!(solve("1 + 2 * 3 + 4 * 5 + 6"), (71, 231));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("1 + (2 * 3) + (4 * (5 + 6))"), (51, 51));
    }

    #[test]
    fn example03() {
        assert_eq!(solve("2 * 3 + (4 * 5)"), (26, 46));
    }

    #[test]
    fn example04() {
        assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), (437, 1445));
    }

    #[test]
    fn example05() {
        assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), (12240, 669060));
    }

    #[test]
    fn example06() {
        assert_eq!(solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), (13632, 23340));
    }
}
