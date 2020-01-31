pub struct Line<'a> {
    line: &'a str,
    position: usize,
}

impl <'a>Line<'a> {
    pub fn new(line: &'a str, position: usize) -> Line<'a> {
        return Line {
            line,
            position,
        }
    }

    pub fn get_line_text(&self) -> &str {
        return self.line;
    }

    pub fn get_line_number(&self) -> usize {
        return self.position;
    }
}