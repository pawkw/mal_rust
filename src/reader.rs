use regex::Regex;

pub fn read_str(input: &String) {
    let pattern = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

    let re = Regex::new(pattern).unwrap();

    let tokens: Vec<_> = re.captures_iter(input).collect();
    for item in tokens {
        println!("{} ", &item[0].trim());
    }
}