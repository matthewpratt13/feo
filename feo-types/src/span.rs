use std::sync::Arc;

pub trait Spanned {
    fn span(&self) -> &Span;
}

#[derive(Default, Debug, Clone)]
pub struct Position {
    input: Arc<String>,
    pos: usize,
}

impl Position {
    pub fn new(input: &str, pos: usize) -> Position {
        input
            .get(pos..)
            .map(|_| Position {
                input: Arc::new(input.to_string()),
                pos,
            })
            .expect("Position out of bounds")
    }

    #[inline]
    pub fn line_col(&self) -> (usize, usize) {
        if self.pos > self.input.len() {
            panic!("Position out of bounds");
        }

        let slice = &self.input[..self.pos];
        let lines = slice.split('\n').collect::<Vec<_>>();
        let line_count = lines.len();
        let last_line_len = lines.last().unwrap_or(&"").chars().count() + 1;

        (line_count, last_line_len)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Span {
    src: Arc<String>,
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(src: &str, start: usize, end: usize) -> Self {
        if start > src.len() {
            panic!("Start position out of bounds")
        }

        if end > src.len() {
            panic!("End position out of bounds")
        }

        let mut start_ = start;
        let mut end_ = end;

        if start > end {
            // swap start and end indexes if needed instead of panicking
            start_ = end;
            end_ = start;
        }

        Self {
            src: Arc::new(src.to_string()),
            start: start_,
            end: end_,
        }
    }

    pub fn source(&self) -> Arc<String> {
        Arc::clone(&self.src)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn as_str(&self) -> &str {
        &self.src[self.start..self.end]
    }
}
