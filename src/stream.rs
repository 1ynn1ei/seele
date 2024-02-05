
pub struct Stream<'a> {
    pub data: &'a [u8],
    idx: usize,
}

impl<'a> Stream<'a> {
    pub fn new(data: &'a[u8]) -> Self {
        Stream {
            data,
            idx: 0
        }
    }

    pub fn cursor(&self) -> usize {
        self.idx
    }

    pub fn advance(&mut self) {
        self.idx += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.data.len()
    }

    pub fn current(&self) -> Option<&u8> {
        self.data.get(self.idx)
    }

    pub fn expect(&self, check: u8) -> bool {
        let cur = self.data.get(self.idx).unwrap();
        *cur == check
    }

    pub fn expect_and_advance(&mut self, check: u8) -> bool {
        if self.expect(check) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn slice(&self, start: usize) -> &'a [u8] {
        &self.data[start..self.idx]
    }
}
