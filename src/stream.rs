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

    pub fn expect(&self, chars: &str) -> bool {
        if chars.len() + self.idx > self.data.len() {
            false
        } else {
            for (offset, byte) in chars.as_bytes().iter().enumerate() {
                println!("{}", offset);
                if &self.data[self.idx+offset] == byte {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }

    }

    pub fn expect_insensitive(&self, chars: &str) -> bool {
        // TODO: obviously needs to be modified for case insensitivity
        if chars.len() + self.idx > self.data.len() {
            println!("test");
            false
        } else {
            for (offset, byte) in chars.as_bytes().iter().enumerate() {
                if &self.data[self.idx+offset] == byte {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }

    }

    pub fn consume(&mut self, chars: &str) {
        self.idx += chars.len() - 1
    }
}
