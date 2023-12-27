use std::sync::Arc;

pub trait Spanned {
    fn span(&self) -> &Span;
}

pub struct Position<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Position<'a> {
    pub fn new(input: &'a str, pos: usize) -> Option<Position<'a>> {
        input.get(pos..).map(|_| Position { input, pos })
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

#[derive(Debug, Clone)]
pub struct Span {
    src: Arc<String>,
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(src: &str, start: usize, end: usize) -> Self {
        Self {
            src: Arc::new(src.to_string()),
            start,
            end,
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
