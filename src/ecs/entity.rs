use super::component::{Component, get_component_t_id};
use std::collections::HashMap;

pub struct Scene {
  component_pools: HashMap<usize, HashMap<usize, &'static dyn Component>>,
  pub entities: HashMap<usize, usize>,
  freed_ids: Vec<usize>
}

impl Scene {
  pub fn new() -> Self {
    Scene { component_pools: HashMap::new(), entities: HashMap::new(), freed_ids: Vec::new() }
  }

  pub fn create_entity(&mut self) -> usize {
    let id;
    if let Some(value) = self.freed_ids.pop() {
      id = value;
    }
    else {
      id = self.entities.len();
    }
    self.entities.insert(id, 0);
    id
  }

  pub fn remove_entity(&mut self, id: usize) {
    self.entities.remove(&id);
    self.freed_ids.push(id);
  }

  pub fn add_component<C: Component + 'static>(&mut self, entity: usize, component: &'static dyn Component) {
    let c_id = get_component_t_id::<C>();
    let c_bit: usize = 1 << c_id;

    let c_mask = self.entities.get_mut(&entity).unwrap();

    if *c_mask & c_bit != 0 { return; }

    *c_mask ^= c_bit;

    match self.component_pools.get_mut(&c_id) {
      Some(pool) => {
        pool.insert(entity, component);
      },
      None => {
        self.component_pools.insert(c_id, HashMap::from([(entity, component)]));
      }
    }
  }

  pub fn remove_component<C: Component + 'static>(&mut self, entity: usize) {
    let c_id = get_component_t_id::<C>();
    let c_bit: usize = 1 << c_id;

    let c_mask = self.entities.get_mut(&entity).unwrap();

    if *c_mask & c_bit == 0 { return; }

    *c_mask ^= c_bit;

    match self.component_pools.get_mut(&c_id) {
      Some(pool) => {
        pool.remove(&entity);
      },
      _ => {}
    }
  }
}
