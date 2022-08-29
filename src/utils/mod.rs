use std::io::Write;

pub fn get_usize_input(prompt: &str) -> usize {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let n: usize = std::io::stdin()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
    n
}

pub fn split_line_in_args(s: String) -> Vec<String> {
    let mut sp: Vec<String> = Vec::new();

    let mut escape_next = false;
    let mut in_double_quote = false;

    let mut tmp: String = String::new();

    for c in s.chars() {
        match c {
            ' ' | '\t' => {
                if in_double_quote {
                    tmp.push(c);
                } else {
                    sp.push(tmp.clone());
                    tmp.clear();
                }
            }
            '"' => {
                if escape_next {
                    tmp.push(c);
                    escape_next = false;
                } else {
                    in_double_quote = !in_double_quote;
                }
            }
            '\\' => {
                if escape_next {
                    tmp.push(c);
                }
                escape_next = !escape_next;
            }
            _ => {
                tmp.push(c);
                escape_next = false;
            }
        }
    }
    if tmp.is_empty() == false {
        sp.push(tmp.clone());
    }

    sp.iter()
        .filter(|s| s.is_empty() == false)
        .cloned()
        .collect()
}
