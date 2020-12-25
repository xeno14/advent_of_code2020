use aoc::read_lines;

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Or(Vec<Rule>),
    Seq(Vec<Rule>), // has next
}

impl Rule {
    pub fn new_char(c: char) -> Rule {
        Rule::Char(c)
    }

    pub fn new_or(or_rules: Vec<Rule>) -> Rule {
        if or_rules.len() <= 1 {
            panic!();
        }
        Rule::Or(or_rules)
    }

    pub fn build(rules: &HashMap<String, String>, ruleno: String) -> Rule {
        let rule = rules.get(&ruleno).unwrap();

        if let Some(rule) = rule.strip_prefix("\"") {
            let c: char = rule.strip_suffix("\"").unwrap().chars().next().unwrap();
            Rule::new_char(c)
        } else if rule.contains("|") {
            let subrules: Vec<&str> = rule.split('|').map(|s| s.trim()).collect();
            Rule::new_or(vec![
                Rule::build_seq(rules, subrules[0]),
                Rule::build_seq(rules, subrules[1]),
            ])
        } else {
            Rule::build_seq(rules, rule)
        }
    }

    /// rule is a sequence of rule numbers
    fn build_seq(rules: &HashMap<String, String>, rule: &str) -> Rule {
        let seq: Vec<Rule> = rule
            .split(' ')
            .map(|ruleno| Rule::build(rules, ruleno.to_string()))
            .collect();
        Rule::Seq(seq)
    }

    pub fn to_regex(&self) -> String {
        match self {
            Rule::Char(c) => c.to_string(),
            Rule::Or(or_rules) => {
                let sub_patterns: Vec<String> = or_rules
                    .iter()
                    .map(|rule| format!("({})", rule.to_regex()))
                    .collect();
                let pattern = sub_patterns.join("|");
                format!("({})", pattern)
            }
            Rule::Seq(seq) => {
                let mut s = "".to_owned();
                for rule in seq {
                    let r = rule.to_regex();
                    s += &r;
                }
                format!("{}", s)
            }
        }
    }
}

fn read_file(filename: &str) -> (HashMap<String, String>, Vec<String>) {
    let mut rules: HashMap<String, String> = HashMap::new();
    let mut strings: Vec<String> = Vec::new();
    let mut parse_rules: bool = true;
    for line in read_lines(filename).into_iter().map(|line| line.unwrap()) {
        if line.len() == 0 {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let parts: Vec<String> = line.split(": ").map(|x| x.to_owned()).collect();
            rules.insert(parts[0].clone(), parts[1].clone());
        } else {
            strings.push(line);
        }
    }

    return (rules, strings);
}

fn main() {
    // let filename = "input/day19-example.txt";
    let filename = "input/day19.txt";
    let (rules, strings) = read_file(filename);

    println!("{:?}", rules);
    println!("{:?}", strings);

    let rule = Rule::build(&rules, "0".to_owned());
    println!("{:#?}", rule);

    let pattern = rule.to_regex();
    let pattern = format!(r"^{}$", pattern);
    println!("{:#?}", pattern);

    let regex = regex::Regex::new(&pattern).unwrap();

    let mut ans = 0;
    for s in strings.iter() {
        let ok = regex.is_match(&s);
        println!("{} {}", s, ok);
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
