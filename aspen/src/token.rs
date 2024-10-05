use std::sync::{atomic::AtomicUsize, LazyLock};

static CURRENT_TOKEN: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    id: usize,
    type_name: &'static str,
}

impl Token {
    pub fn new<C>() -> Self {
        let id = CURRENT_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Token {
            id,
            type_name: std::any::type_name::<C>(),
        }
    }
}
