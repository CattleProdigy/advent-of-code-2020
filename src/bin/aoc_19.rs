use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Rule {
    Char { c: char },
    Other { r: Vec<Vec<usize>> },
}

fn parse_rule_ln(input: &str) -> (usize, Rule) {
    let col_pos = input.find(':').unwrap();
    let idx_slice = &input[..col_pos];
    let idx = idx_slice.parse::<usize>().unwrap();

    let rem_slice = &input[col_pos + 1..];
    let maybe_quote = rem_slice.find('\"');
    let r = match maybe_quote {
        Some(_) => Rule::Char {
            c: rem_slice.chars().find(|x| x.is_ascii_alphabetic()).unwrap(),
        },
        None => Rule::Other {
            r: rem_slice
                .split('|')
                .map(|sub| {
                    sub.split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        },
    };

    (idx, r)
}

fn to_regex(input: &Vec<(usize, Rule)>) -> HashMap<usize, String> {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut regexs: HashMap<usize, String> = HashMap::new();

    queue.push_front(0);

    while !queue.is_empty() {
        let cur = queue.pop_back().unwrap();
        println!("{}", cur);
        if regexs.contains_key(&cur) {
            println!("skippingrule: {}", cur);
            continue;
        }
        let (rule_idx, input_rule) = &input[cur];
        assert_eq!(*rule_idx, cur);
        match input_rule {
            Rule::Char { c } => {
                println!("base_rule {} {} {}", cur, *rule_idx, c);
                regexs.insert(*rule_idx, c.to_string());
            }
            Rule::Other { r } => {
                let mut satisfied: bool = true;
                let mut children: Vec<usize> = Vec::new();
                if *rule_idx == 0 {
                    if !(regexs.contains_key(&42)) {
                        println!("missing rule: {}", 42);
                        satisfied = false;
                        children.push(42);
                    }
                    if !(regexs.contains_key(&31)) {
                        println!("missing rule: {}", 31);
                        satisfied = false;
                        children.push(31);
                    }
                } else if *rule_idx == 8 {
                } else if *rule_idx == 11 {
                } else {
                    for group in r.iter() {
                        for idx in group.iter() {
                            if !(regexs.contains_key(&idx)) {
                                println!("gmissing rule: {}", idx);
                                satisfied = false;
                                children.push(*idx);
                            }
                        }
                    }
                }
                if !satisfied {
                    queue.push_back(cur);
                    for c in children.iter() {
                        queue.push_back(*c);
                    }
                } else {
                    if *rule_idx == 0 {
                        let mut regex_str: String = "^(?:".to_string();
                        regex_str.push_str(&regexs[&42]);
                        regex_str.push_str(")+");
                        regex_str.push_str("(?:");
                        for i in 1..5 {
                            for _ in 0..i {
                                regex_str.push_str(&regexs[&42]);
                            }
                            for _ in 0..i {
                                regex_str.push_str(&regexs[&31]);
                            }
                            regex_str.push('|');
                        }
                        for _ in 0..6 {
                            regex_str.push_str(&regexs[&42]);
                        }
                        for _ in 0..6 {
                            regex_str.push_str(&regexs[&31]);
                        }
                        regex_str.push_str(")$");

                        regexs.insert(*rule_idx, regex_str);

                    // if *rule_idx == 8 {
                    //     let mut regex_str: String = "(?:".to_string();
                    //     regex_str.push_str(&regexs[&42]);
                    //     regex_str.push_str(")+");
                    //     regexs.insert(*rule_idx, regex_str);
                    // } else if *rule_idx == 11 {
                    //     let mut regex_str: String = "(?:".to_string();
                    //     for i in 1..15 {
                    //         for _ in 0..i {
                    //             regex_str.push_str(&regexs[&42]);
                    //         }
                    //         for _ in 0..i {
                    //             regex_str.push_str(&regexs[&31]);
                    //         }
                    //         regex_str.push('|');
                    //     }
                    //     for _ in 0..16 {
                    //         regex_str.push_str(&regexs[&42]);
                    //     }
                    //     for _ in 0..16 {
                    //         regex_str.push_str(&regexs[&31]);
                    //     }
                    //     regex_str.push(')');

                    //     regexs.insert(*rule_idx, regex_str);
                    } else {
                        let mut regex_str: String = "(?:".to_string();
                        let mut group_itr = r.iter();
                        regex_str.push_str(
                            &group_itr
                                .next()
                                .unwrap()
                                .iter()
                                .map(|ri| regexs[ri].as_str())
                                .collect::<String>(),
                        );

                        while let Some(g) = group_itr.next() {
                            regex_str.push('|');
                            regex_str.push_str(
                                &g.iter().map(|ri| regexs[ri].as_str()).collect::<String>(),
                            );
                        }
                        regex_str.push(')');
                        regexs.insert(*rule_idx, regex_str);
                    }
                }
            }
        }
    }

    for (i, r) in regexs.iter() {
        println!("{}: {}", i, r.as_str());
    }

    regexs
}
fn match_iterative(
    input: &str,
    target_rule: usize,
    part2: bool,
    rules: &Vec<(usize, Rule)>,
) -> bool {
    println!("matching: {}", input);
    let (matched, advanced) = match_iterative_impl(input, target_rule, part2, rules);
    matched && advanced == input.len()
}

fn match_iterative_impl(
    input: &str,
    target_rule: usize,
    part2: bool,
    rules: &Vec<(usize, Rule)>,
) -> (bool, usize) {
    if input.is_empty() {
        return (false, 0);
    }

    let mut map: HashMap<(usize, usize), (bool, usize)> = HashMap::new();

    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_back((0, target_rule));
    while !stack.is_empty() {
        let (offset, rule_idx) = stack.pop_back().unwrap();

        println!("{} {} {}", offset, rule_idx, input.len());

        if map.contains_key(&(offset, rule_idx)) {
            // println!("skipping");
            continue;
        }
        if offset >= input.len() {
            map.insert((offset, rule_idx), (false, 0));
            continue;
        }

        let input_slice = &input[offset..];
        let mut ci = input_slice.char_indices();
        let (_idx, rule) = &rules[rule_idx];
        assert_eq!(*_idx, rule_idx);
        match rule {
            Rule::Char { c } => {
                let (_i, rc) = ci.next().unwrap();
                map.insert((offset, rule_idx), (rc == *c, offset + 1));
            }
            Rule::Other { r } => {
                let mut all_visited = true;
                let mut matched: Option<usize> = None;
                let mut children: Vec<(usize, usize)> = Vec::new();

                if rule_idx == 8 && part2 {
                    let mut group_matched = false;
                    let mut group_children_visited = true;
                    let mut last_offset = offset;
                    let idx = 42;
                    loop {
                        if map.contains_key(&(last_offset, idx)) {
                            let (matched, match_offset) = map[&(last_offset, idx)];
                            if !matched {
                                break;
                            } else {
                                last_offset = match_offset;
                                group_matched = true;
                            }
                        } else {
                            // println!("missing child: {}", idx);
                            group_children_visited = false;
                            children.push((last_offset, idx));
                            break;
                        }
                    }

                    if group_matched {
                        println!("rule 8 matched");
                        matched = Some(last_offset);
                    }

                    if !group_children_visited {
                        println!("rule 8 needs children");
                        all_visited = false;
                    }
                } else if rule_idx == 11 && part2 {
                    let mut group_42_matched = 0;
                    let mut group_31_matched = 0;
                    let mut group_children_visited = true;
                    let mut last_offset = offset;
                    {
                        let idx = 42;
                        loop {
                            if map.contains_key(&(last_offset, idx)) {
                                let (matched, match_offset) = map[&(last_offset, idx)];
                                if !matched {
                                    break;
                                } else {
                                    last_offset = match_offset;
                                    group_42_matched += 1;
                                    if last_offset >= input.len() {
                                        break;
                                    }
                                }
                            } else {
                                group_children_visited = false;
                                group_42_matched = 0;
                                children.push((last_offset, idx));
                                break;
                            }
                        }
                    }
                    if group_42_matched > 0 && last_offset < input.len() {
                        let idx = 31;
                        for _ in 0..group_42_matched {
                            if map.contains_key(&(last_offset, idx)) {
                                let (matched, match_offset) = map[&(last_offset, idx)];
                                if !matched {
                                    break;
                                } else {
                                    last_offset = match_offset;
                                    group_31_matched += 1;
                                }
                            } else {
                                // println!("missing child: {}", idx);
                                group_children_visited = false;
                                group_31_matched = 0;
                                children.push((last_offset, idx));
                                break;
                            }
                        }
                    }

                    if group_31_matched >= 1 && (group_42_matched == group_31_matched) {
                        println!("rule 11 matched");
                        matched = Some(last_offset);
                    }

                    if !group_children_visited {
                        println!("rule 11 needs children");
                        all_visited = false;
                    }
                } else {
                    for group in r.iter() {
                        let mut group_matched = true;
                        let mut group_children_visited = true;
                        let mut last_offset = offset;
                        for idx in group.iter() {
                            if map.contains_key(&(last_offset, *idx)) {
                                let (matched, match_offset) = map[&(last_offset, *idx)];
                                if !matched {
                                    group_matched = false;
                                    break;
                                }
                                last_offset = match_offset;
                            } else {
                                // println!("missing child: {}", idx);
                                group_children_visited = false;
                                group_matched = false;
                                children.push((last_offset, *idx));
                                break;
                            }
                        }

                        if group_matched {
                            matched = Some(last_offset);
                            break;
                        }

                        if !group_children_visited {
                            all_visited = false;
                            break;
                        }
                    }
                }

                if matched.is_some() {
                    // println!("marking matched: {} {}", offset, rule_idx);
                    map.insert((offset, rule_idx), (true, matched.unwrap()));
                } else if all_visited {
                    println!("marking unmatched: {} {}", offset, rule_idx);
                    map.insert((offset, rule_idx), (false, 0));
                }

                if !children.is_empty() {
                    stack.push_back((offset, rule_idx));

                    for c in children {
                        stack.push_back(c);
                    }
                }
            }
        }
    }

    map[&(0, target_rule)]
}
fn match_recursive(input: &str, target_rule: usize, rules: &Vec<(usize, Rule)>) -> bool {
    let (matched, advanced) = match_recursive_impl(input, target_rule, rules);
    matched && advanced == input.len()
}

fn match_recursive_impl(
    input: &str,
    target_rule: usize,
    rules: &Vec<(usize, Rule)>,
) -> (bool, usize) {
    if input.is_empty() {
        return (false, 0);
    }
    let mut ci = input.char_indices();
    let (_idx, rule) = &rules[target_rule];
    match rule {
        Rule::Char { c } => {
            let (_i, rc) = ci.next().unwrap();
            if rc == *c {
                return (true, 1);
            } else {
                return (false, 0);
            }
        }
        Rule::Other { r } => {
            let mut advance_total: usize = 0;
            let matches = r.iter().any(|group| {
                advance_total = 0;
                group.iter().all(|rule| {
                    let (matched, advance) =
                        match_recursive_impl(&input[advance_total..], *rule, &rules);
                    advance_total += advance;
                    matched
                })
            });
            return (matches, if matches { advance_total } else { 0 });
        }
    }
}

fn parse(input: &str) -> (Vec<(usize, Rule)>, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let mut rules = iter
        .next()
        .unwrap()
        .lines()
        .map(|l| parse_rule_ln(l))
        .collect::<Vec<_>>();
    rules.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let msgs = iter.next().unwrap().lines().collect::<Vec<&str>>();
    (rules, msgs)
}

fn part2_adjustment(rules: &Vec<(usize, Rule)>) -> Vec<(usize, Rule)> {
    let mut new_rules = rules.to_vec();
    {
        let (idx, rule) = &mut new_rules[8];
        assert_eq!(*idx, 8);
        *rule = Rule::Other {
            r: vec![vec![42], vec![42, 8]],
        };
    }
    {
        let (idx, rule) = &mut new_rules[11];
        assert_eq!(*idx, 11);
        *rule = Rule::Other {
            r: vec![vec![42, 31], vec![42, 11, 31]],
        };
    }

    new_rules
}

fn match_exact(input: &str, re: &Regex) -> bool {
    match re.find(input) {
        Some(m) => m.start() == 0 && m.end() == input.len(),
        None => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let test_str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
    let (rules, msgs) = parse(&file_contents);
    {
        println!("{:?}", rules);

        let msg_0_count = msgs
            .iter()
            .filter(|&m| match_iterative(m, 0, false, &rules))
            .count();
        println!("p1: {}", msg_0_count);
    }

    {
        let rules2 = part2_adjustment(&rules);
        let r = to_regex(&rules2);
        println!("rules{:?}", rules2);
        let msg_0_count = msgs
            .iter()
            .filter(|&m| match_exact(m, &Regex::new(&r[&0]).unwrap()))
            .count();
        println!("p2: {}", msg_0_count);

        // let msg_0_count = msgs
        //     .iter()
        //     .filter(|&m| match_iterative(m, 0, true, &rules2))
        //     .count();
        // println!("p2: {}", msg_0_count);
    }
}
