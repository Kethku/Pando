use std::sync::{atomic::AtomicUsize, LazyLock};

static CURRENT_TOKEN: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
static DEFAULT_TOKEN: LazyLock<Token> = LazyLock::new(|| Token::new());

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    id: usize,
}

impl Token {
    pub fn new() -> Self {
        let id = CURRENT_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Token { id }
    }
}

impl Default for Token {
    fn default() -> Self {
        *DEFAULT_TOKEN
    }
}
