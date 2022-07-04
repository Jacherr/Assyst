use std::{rc::Rc, cell::RefCell};

use crate::guild_cache::GuildCache;

pub struct State {
    pub guild_cache: GuildCache
}
impl State {
    pub fn new() -> State {
        State { 
            guild_cache: GuildCache::new()
        }
    }
}

pub type SharedState = Rc<RefCell<State>>;