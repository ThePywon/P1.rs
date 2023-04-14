use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
pub struct EventManager<K: Eq + Hash, T> {
  events: HashMap<K, Vec<fn(&T)>>
}

impl<K: Eq + Hash, T> EventManager<K, T> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    EventManager { events: HashMap::new() }
  }

  #[allow(dead_code)]
  pub fn on(&mut self, key: K, func: fn(&T)) {
    match self.events.get_mut(&key) {
      Some(callbacks) => callbacks.push(func),
      None => { self.events.insert(key, vec![func]); }
    }
  }

  #[allow(dead_code)]
  pub fn emit(&self, key: K, value: T) {
    match self.events.get(&key) {
      Some(callbacks) =>
        for callback in callbacks.iter() {
          callback(&value);
        },
      _ => {}
    }
  }
}

#[allow(dead_code)]
pub struct KeyOnlyEventManager<K: Eq + Hash> {
  events: HashMap<K, Vec<fn()>>
}

impl<K: Eq + Hash> KeyOnlyEventManager<K> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    KeyOnlyEventManager { events: HashMap::new() }
  }

  #[allow(dead_code)]
  pub fn on(&mut self, key: K, func: fn()) {
    match self.events.get_mut(&key) {
      Some(callbacks) => callbacks.push(func),
      None => { self.events.insert(key, vec![func]); }
    }
  }

  #[allow(dead_code)]
  pub fn emit(&self, key: K) {
    match self.events.get(&key) {
      Some(callbacks) =>
        for callback in callbacks.iter() {
          callback();
        },
      _ => {}
    }
  }
}
