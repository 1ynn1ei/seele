pub struct Stream<'stream> {
    pub data: &'stream Vec<u8>,
    idx: usize,
}

impl<'stream> Stream<'stream> {
    pub fn new(data: &'stream Vec<u8>) -> Self {
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

    pub fn reconsume(&mut self) {
        self.idx -= 1;
    }

    pub fn is_eof(&self) -> bool {
        self.idx >= self.data.len()
    }

    pub fn current(&self) -> &'stream u8 {
        &self.data[self.idx]
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

    pub fn slice(&self, start: usize) -> &'stream [u8] {
        &self.data[start..self.idx]
    }
}
