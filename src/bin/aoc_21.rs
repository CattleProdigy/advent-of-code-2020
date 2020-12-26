use std::collections::HashMap;
use std::env;
use std::fs;

fn parse(input: &str) {
    let mut all_allergens: Vec<&str> = Vec::new();
    let mut ingredient_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut allergen_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut foods: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for l in input.lines() {
        let allergen_delimiter = l.find(" (contains ").unwrap_or(l.len());

        let ingredients_slice = &l[..allergen_delimiter];
        let ingredients = ingredients_slice.split(" ").collect::<Vec<_>>();

        let allergen_slice = &l[allergen_delimiter + " (contains ".len()..l.len() - 1];
        let allergens = allergen_slice.split(", ").collect::<Vec<_>>();

        for i in ingredients.iter() {
            (*ingredient_map.entry(i).or_insert(Vec::new())).append(&mut allergens.clone());
        }
        for a in allergens.iter() {
            (*allergen_map.entry(a).or_insert(Vec::new())).append(&mut ingredients.clone());
        }

        all_allergens.append(&mut allergens.clone());
        foods.push((ingredients, allergens));
    }
    all_allergens.sort();
    all_allergens.dedup();

    for (_, v) in ingredient_map.iter_mut() {
        v.sort();
        v.dedup();
    }
    for (_, v) in allergen_map.iter_mut() {
        v.sort();
        v.dedup();
    }

    let depth = all_allergens.len();

    let mut feasible_assignements: Vec<Vec<(&str, &str)>> = Vec::new();

    let mut assignments_in_prog: Vec<(&str, &str)> = Vec::new();

    let mut stack: Vec<(&str, &str, usize)> = Vec::new();
    {
        let allergen_to_assign = all_allergens[0];
        println!("{}", allergen_to_assign);
        println!("{:?}", allergen_map);
        let options = &allergen_map[allergen_to_assign];
        for o in options.iter() {
            stack.push((allergen_to_assign, o, 0));
        }
    }

    println!("==================");

    while !stack.is_empty() {
        let (all, ing, d) = stack.pop().unwrap();

        assignments_in_prog.resize(d, ("", ""));
        assignments_in_prog.push((all, ing));

        // evaluate rules
        let mut violation: bool = false;
        for (mut i, mut a) in foods.iter().cloned() {
            for (ass_all, ass_ing) in assignments_in_prog.iter() {
                if a.iter().find(|x| x == &ass_all).is_some() {
                    if i.iter().find(|i| i == &ass_ing).is_none() {
                        violation = true;
                        break;
                    }
                    i.retain(|ii| ii != ass_ing);
                    a.retain(|aa| aa != ass_all);
                }
            }
            if violation {
                break;
            }
        }

        if violation {
            continue;
        }

        // if passed rules and depth indicates that we've finished assigning
        // then we have a feasible soln use log it.
        if d == depth - 1 {
            println!("{:?}", assignments_in_prog);
            feasible_assignements.push(assignments_in_prog.clone());
            continue;
        }

        // pick next value to assign and push all the candidates
        let allergen_to_assign = all_allergens[d + 1];
        let mut options = allergen_map[allergen_to_assign].clone();
        options.retain(|x| assignments_in_prog.iter().find(|(_, i)| x == i).is_none());
        for o in options.iter() {
            stack.push((allergen_to_assign, o, d + 1));
        }
    }

    let impossible_ingredients = ingredient_map
        .keys()
        .filter(|i| {
            feasible_assignements[0]
                .iter()
                .find(|ii| *i == &ii.1)
                .is_none()
        })
        .collect::<Vec<_>>();

    let impossible_count = foods
        .iter()
        .map(|(is, _)| {
            is.iter()
                .filter(|ii| {
                    impossible_ingredients
                        .iter()
                        .find(|iii| iii == &ii)
                        .is_some()
                })
                .count()
        })
        .sum::<usize>();
    println!("p1: {:?}", impossible_count);

    let mut string = feasible_assignements[0]
        .iter()
        .map(|(_, is)| *is)
        .collect::<Vec<&str>>();
    string.sort_by_key(|k| {
        feasible_assignements[0]
            .iter()
            .find(|(_, i)| k == i)
            .unwrap()
            .0
    });
    let canon_string = string
        .iter()
        .map(|s| {
            let mut s = s.to_string();
            s.push(',');
            s
        })
        .collect::<String>();

    println!("p2: {:?}", canon_string);
}

fn main() {
    let example = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

    parse(example);

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    parse(&file_contents);
}
