use std::borrow::Borrow;
use html5ever::tokenizer::{
    TokenSink,
    TokenSinkResult,
    Token,
    TagToken,
    Tag,
    TagKind
};

#[derive(Default, Debug)]
pub struct LinkQueue {
    pub links: Vec<String>,
}

impl TokenSink for &mut LinkQueue {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            TagToken(ref tag @ Tag { kind: TagKind::StartTag, .. }) => {
                if tag.name.as_ref() == "a" {
                    for attribute in tag.attrs.iter() {
                        if attribute.name.local.as_ref() == "href" {
                            let url_str: &[u8] = attribute.value.borrow();
                            self.links.push(String::from_utf8_lossy(url_str).into_owned());
                        }
                    }
                }
            },
            _ => {}
        }
        TokenSinkResult::Continue
    }
}
