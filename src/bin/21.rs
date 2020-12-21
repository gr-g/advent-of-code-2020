use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: Vec<String>,
}

impl Food {
    fn create_from(s: &str) -> Food {
        let mut ingredients = HashSet::new();
        let mut allergens = Vec::new();
        let mut allergens_part = false;
        for word in s.split_whitespace() {
            if word == "(contains" {
                allergens_part = true
            } else if allergens_part {
                allergens.push(word.trim_end_matches(|c| c == ',' || c == ')').to_string());
            } else {
                ingredients.insert(word.to_string());
            }
        }
        Food{ ingredients, allergens }
    }
}

fn solve(input: &str) -> (usize, String) {
    let food: Vec<_> = input.lines().map(|line| Food::create_from(line)).collect();

    // Store a map between an allergen and the ingredients that may hold it.
    let mut potentially_dangerous: HashMap<String, HashSet<String>> = HashMap::new();

    // The ingredient associated with a certain allergen must appear in all
    // the food items with that allergen.
    for Food{ ingredients, allergens } in &food {
        for a in allergens {
            match potentially_dangerous.entry(a.to_string()) {
                Entry::Occupied(mut e) => {
                    e.insert(e.get().intersection(ingredients).cloned().collect());
                },
                Entry::Vacant(e) => {
                    e.insert(ingredients.clone());
                },
            }
        }
    }

    //println!("Potentially dangerous food: {:?}", potentially_dangerous);

    let safe = food.iter()
        .flat_map(|Food{ ingredients, .. }| ingredients.iter())
        .filter(|i| potentially_dangerous.values().all(|ing_list| !ing_list.contains(*i)))
        .count();

    let mut dangerous = Vec::new();

    while let Some((a, _)) = potentially_dangerous.iter().find(|(_, list)| list.len() == 1) {
        let allergen = a.clone();
        let ing_list = potentially_dangerous.remove(&allergen).unwrap();
        let ingredient = ing_list.into_iter().nth(0).unwrap();

        for ing_list in potentially_dangerous.values_mut() {
            ing_list.remove(&ingredient);
        }

        dangerous.push((allergen, ingredient));
    }

    if !potentially_dangerous.is_empty() {
        panic!("the problem does not have a unique solution");
    }

    //println!("Dangerous food: {:?}", dangerous);

    dangerous.sort();
    let dangerous_list = dangerous.into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>()
        .join(",");

    (safe, dangerous_list)
}

fn main() {
    let input = std::fs::read_to_string("input/21.txt").unwrap();
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
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"), (5, "mxmxvkd,sqjhc,fvjkl".to_string()));
    }
}
