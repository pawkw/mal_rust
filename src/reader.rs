use regex::Regex;

struct Reader {
    token_vector: Vec<String>,
    cursor: usize,
}

impl Reader {
    fn new(data: Vec<String>) -> Self {
        Self {
            token_vector: data.clone(),
            cursor: 0,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.token_vector.get(self.cursor)
    }

    fn read(&mut self) -> Option<&String> {
        self.cursor += 1;
        self.peek()
    }
}

pub fn read_str(input: &String) {
    let token_reader = Reader::new(tokenize(&input));
    for item in token_reader.token_vector {
        println!("{}", item);
    }
}

fn tokenize(input: &String) -> Vec<String> {
    const PATTERN: &str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;

    let re = Regex::new(PATTERN).unwrap();
    re
        .captures_iter(input)
        .map(|x| x[0].trim().to_string())
        .collect()
}

