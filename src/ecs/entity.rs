use super::component::{Component, get_component_t_id};
use std::collections::HashMap;

pub struct Scene {
  component_pools: HashMap<usize, HashMap<usize, Box<dyn Component>>>,
  pub entities: HashMap<usize, usize>,
  next_entity_id: usize
}

impl Scene {
  pub fn new() -> Self {
    Scene { component_pools: HashMap::new(), entities: HashMap::new(), next_entity_id: 0 }
  }

  fn create_entity_id(&mut self) -> usize {
    let result = self.next_entity_id;
    self.next_entity_id += 1;
    result
  }

  pub fn create_entity(&mut self) -> usize {
    let id = self.create_entity_id();
    self.entities.insert(id, 0);
    id
  }

  pub fn add_component<C: Component + 'static>(&mut self, entity: usize, component: Box<dyn Component>) {
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