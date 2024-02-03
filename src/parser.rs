use crate::dom;

struct Stream<'a> {
    data: &'a [u8],
    idx: usize,
}

impl<'a> Stream<'a> {
    pub fn new(data: &'a[u8]) -> Self {
        Stream {
            data,
            idx: 0
        }
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

    pub fn expect(&self, check: &u8) -> bool {
        let cur = self.data.get(self.idx).unwrap();
        cur == check
    }

    pub fn slice(&self, start: usize) -> &'a [u8] {
        &self.data[start..self.idx]
    }
}

pub struct Parser<'a> {
    stream: Stream<'a>,
}

impl<'a> Parser<'a> {
    fn new (data: &'a [u8]) -> Self {
        Self {
            stream: Stream::new(data)
        }
    }


    fn read_tag(&mut self) -> dom::Node {
        // we have read a b'<' already
        let attributes = dom::AttrMap::new();
        self.skip_whitespace().unwrap();
        let tag_name = self.read_to_or(b'>', b' ');
        self.skip_whitespace().unwrap();
        // are we at end of tag?
        // yes, attributes is empty
        // no keep going until we are at end of tag
        dom::Node::elem(tag_name, attributes, Vec::new())
    }

    fn skip_whitespace(&mut self) -> Option<()> {
        loop {
            let char = self.stream.current()?;
            match char {
                b' ' | b'\n' => self.stream.advance(),
                _ => return Some(())
            }
        }
    }

    fn read_identifier(&mut self) -> &'a [u8] {
        let start = self.stream.idx;
        loop {
            if let Some(char) = self.stream.current() {
                match char {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                        self.stream.advance();
                    },
                    _ => {
                        return self.stream.slice(start);
                    }
                }
            }
        }
    }

    fn read_to(&mut self, needle: u8) -> &'a [u8] {
        let start = self.stream.idx;
        loop {
            if let Some(char) = self.stream.current() {
                if *char == needle {
                    return self.stream.slice(start)
                } else {
                    self.stream.advance();
                }
            }
        }
    }

    fn read_to_or(&mut self, needle1: u8, needle2: u8) -> &'a [u8] {
        let start = self.stream.idx;
        loop {
            if let Some(char) = self.stream.current() {
                if *char == needle1 || *char == needle2 {
                    return self.stream.slice(start)
                } else {
                    self.stream.advance();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_skipping_whitespace() {
        let test_value = "    t".as_bytes();
        let mut parser = Parser::new(test_value);
        parser.skip_whitespace();
        assert_eq!(b't', parser.stream.data[parser.stream.idx]);
    }

    #[test]
    fn test_read_to_quote() {
        let test_value = "testing value\"".as_bytes();
        let mut parser = Parser::new(test_value);
        assert_eq!("testing value".as_bytes(), parser.read_to(b'"'));
    }

    #[test]
    fn test_read_tag() {
        let test_value = "p >".as_bytes();
        let mut parser = Parser::new(test_value);
        let node = parser.read_tag();
        assert_eq!(node.get_tag_name(), "p".as_bytes());
    }
    #[test]
    fn test_read_attr() {
        let test_value = "<div class=\"test\">";
        todo!();
    }
}
