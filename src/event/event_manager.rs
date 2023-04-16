use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
pub struct EventManager<K: Eq + Hash, T> {
  events: HashMap<K, Vec<fn(&T)>>,
  funnels: Vec<fn(&K, &T)>
}

impl<K: Eq + Hash, T> EventManager<K, T> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    EventManager { events: HashMap::new(), funnels: Vec::new() }
  }

  #[allow(dead_code)]
  pub fn on(&mut self, key: K, func: fn(&T)) {
    match self.events.get_mut(&key) {
      Some(callbacks) => callbacks.push(func),
      None => { self.events.insert(key, vec![func]); }
    }
  }

  #[allow(dead_code)]
  pub fn funnel(&mut self, func: fn(&K, &T)) {
    self.funnels.push(func);
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
    for funnel in self.funnels.iter() {
      funnel(&key, &value);
    }
  }
}

#[allow(dead_code)]
pub struct KeyOnlyEventManager<K: Eq + Hash> {
  events: HashMap<K, Vec<fn()>>,
  funnels: Vec<fn(&K)>
}

impl<K: Eq + Hash> KeyOnlyEventManager<K> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    KeyOnlyEventManager { events: HashMap::new(), funnels: Vec::new() }
  }

  #[allow(dead_code)]
  pub fn on(&mut self, key: K, func: fn()) {
    match self.events.get_mut(&key) {
      Some(callbacks) => callbacks.push(func),
      None => { self.events.insert(key, vec![func]); }
    }
  }

  #[allow(dead_code)]
  pub fn funnel(&mut self, func: fn(&K)) {
    self.funnels.push(func);
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
    for funnel in self.funnels.iter() {
      funnel(&key);
    }
  }
}
