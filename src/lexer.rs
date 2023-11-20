#[derive(Debug)]
pub struct Lexer {
    src: Vec<Line>,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    line: String,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        let mut lines = Vec::new();
        for line in src.lines() {
            // strip whitespaces
            if line.trim().is_empty() {
                continue;
            }
            lines.push(Line { line: line.trim().to_string() });
        }
        Lexer { src: lines, pos: 0 }
    }

    pub fn next(&mut self) -> Option<&Line> {
        if self.pos < self.src.len() {
            let line = &self.src[self.pos];
            self.pos += 1;
            Some(line)
        } else {
            None
        }
    }
    
    pub fn peek(&self) -> Option<&Line> {
        if self.pos < self.src.len() {
            Some(&self.src[self.pos])
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lexer_instruction_lines() {
        // define multi line string
        let src = r#"li $t0, 123
            li $t1, 456"#;


        let mut lexer = Lexer::new(src);
        println!("src: {:#?}", lexer);
        let line = lexer.next().unwrap();
        assert_eq!(line.line, "li $t0, 123");
        let line = lexer.next().unwrap();
        assert_eq!(line.line, "li $t1, 456");
        assert_eq!(lexer.next(), None);
    }
}

