use uuid::Uuid;
use super::component::{Component, get_component_id};
use std::collections::HashMap;

pub struct Entity {
  scene: &'static mut Scene,
  pub id: Uuid,
  component_mask: usize
}

pub struct Scene {
  pub component_pools: HashMap<usize, HashMap<Uuid, Box<dyn Component>>>,
  pub entities: Vec<Entity>
}

impl Scene {
  pub fn new() -> Self {
    Scene { component_pools: HashMap::new(), entities: Vec::new() }
  }

  fn add_component(&mut self, c_id: usize, entity: Uuid, component: Box<dyn Component + 'static>) {
    match self.component_pools.get_mut(&c_id) {
      Some(pool) => {
        pool.insert(entity, component);
      },
      None => {
        self.component_pools.insert(c_id, HashMap::from([(entity, component)]));
      }
    }
  }

  fn remove_component(&mut self, c_id: usize, entity: Uuid) {
    match self.component_pools.get_mut(&c_id) {
      Some(pool) => {
        pool.remove(&entity);
      },
      _ => {}
    }
  }
}

impl Entity {
  pub fn new(scene: &'static mut Scene) -> Self {
    Entity { scene, id: Uuid::new_v4(), component_mask: 0 }
  }

  pub fn add_component<C: Component + 'static>(&mut self, component: C) {
    let c_id = get_component_id::<C>();
    let c_bit: usize = 1 << c_id;

    if self.component_mask & c_bit == 0 {
      self.component_mask ^= c_bit;
      self.scene.add_component(c_id, self.id, Box::new(component));
    }
  }

  pub fn remove_component<C: Component + 'static>(&mut self) {
    let c_id = get_component_id::<C>();
    let c_bit: usize = 1 << c_id;

    if self.component_mask & c_bit != 0 {
      self.component_mask ^= c_bit;
      self.scene.remove_component(c_id, self.id);
    }
  }
}
