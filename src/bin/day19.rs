use aoc::read_lines;

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Or(Box<Rule>, Box<Rule>),
    Seq(Vec<Rule>), // has next
}

impl Rule {
    pub fn new_char(c: char) -> Rule {
        Rule::Char(c)
    }

    pub fn new_or(left: Rule, right: Rule) -> Rule {
        Rule::Or(Box::new(left), Box::new(right))
    }

    pub fn build(rules: &HashMap<String, String>, ruleno: String) -> Rule {
        let rule = rules.get(&ruleno).unwrap();

        if let Some(rule) = rule.strip_prefix("\"") {
            let c: char = rule.strip_suffix("\"").unwrap().chars().next().unwrap();
            Rule::new_char(c)
        } else if rule.contains("|") {
            let subrules: Vec<&str> = rule.split('|').map(|s| s.trim()).collect();
            Rule::new_or(
                Rule::build_seq(rules, subrules[0]),
                Rule::build_seq(rules, subrules[1]),
            )
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

    fn do_match(&self, s: &str) -> (bool, usize) {
        match self {
            Rule::Char(c) => {
                if let Some(other) = s.chars().nth(0) {
                    if *c == other {
                        return (true, 1);
                    }
                }
                (false, 0)
            }
            Rule::Or(left, right) => {
                let (left_ok, delta) = left.do_match(s);
                if left_ok {
                    (true, delta)
                } else {
                    right.do_match(s)
                }
            }
            Rule::Seq(seq) => {
                let mut t = s;
                let mut tot_delta = 0;
                for rule in seq {
                    let (ok, delta) = rule.do_match(t);
                    if !ok {
                        return (false, 0);
                    }
                    t = &t[delta..];
                    tot_delta += delta;
                }
                (true, tot_delta)
            }
        }
    }

    pub fn match_str(&self, s: &str) -> bool {
        let (ok, delta) = self.do_match(s); 
        ok && delta == s.len()
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

    let mut ans = 0;
    for s in strings.iter() {
        let ok = rule.match_str(s);
        println!("{} {}", s, ok);
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
