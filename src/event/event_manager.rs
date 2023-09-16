use std::collections::HashMap;
use std::hash::Hash;

pub struct EventManager<K: Eq + Hash, T> {
  events: HashMap<K, Vec<fn(&T)>>
}

impl<K: Eq + Hash, T> EventManager<K, T> {
  pub fn new() -> Self {
    EventManager { events: HashMap::new() }
  }

  pub fn on(&mut self, key: K, func: fn(&T)) {
    match self.events.get_mut(&key) {
      Some(callbacks) => callbacks.push(func),
      None => { self.events.insert(key, vec![func]); }
    }
  }

  pub fn emit(&self, key: K, value: T) {
    match self.events.get(&key) {
      Some(callbacks) =>
        for callback in callbacks.iter() {
          callback(&value);
        },
      None => {}
    }
  }
}
