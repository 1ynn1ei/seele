//https://html.spec.whatwg.org/multipage/parsing.html#tokenization
struct DocType<'stream> {
    name: Option<&'stream [u8]>,
    public_id: Option<&'stream [u8]>,
    system_id: Option<&'stream [u8]>,
    force_quirks: bool,
}

impl<'stream> DocType<'stream> {
    pub fn new() -> Self {
        Self {
            name: None,
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }
}

pub enum Tokens<'stream> {
    DOCTYPE(DocType<'stream>),
    StartTag,
    EndTag,
    Comment(&'stream [u8]),
    Character(&'stream [u8]),
    EndOfFile,
}
