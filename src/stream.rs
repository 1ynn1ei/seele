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
}
