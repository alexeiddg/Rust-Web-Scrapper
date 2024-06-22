use html5ever::tokenizer::{
    BufferQueue,
    Tag,
    TagKind,
    TagToken,
    Token,
    TokenSink,
    TokenSinkResult,
    Tokenizer,
    TokenizerOpts,
};

use url::{
    ParseError,
    Url,
};

use std::borrow::Borrow;

// Store Links in a vector
#[derive(Default, Debug)]
struct LinkQueue {
    links: Vec<String>
}

impl TokenSink for LinkQueue {
    type Handle = ();
    fn process_token( &mut self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            TagToken( ref tag @ Tag { kind: TagKind::StartTag, .. }, ) => {
                if tag.name.as_ref() = "a" {
                    for attribute in tag.attrs.iter() {
                        if attribute.name.local.as_ref() = "href" {
                            let url_str: &[u8] = attribute.value.borrow();
                            self.links.push( String::from_utf8_lossy(url_str).into_owned() )
                        }
                    }
                }
            }
            _ => {}
        }
        TokenSinkResult::Continue
    }
}