use std::any::TypeId;

pub trait Component {
  fn get_entity_id(&self) -> usize;
}

static mut REGISTERED_TYPE_IDS: Vec<TypeId> = vec![];

pub fn get_component_t_id<T: Component + 'static>() -> usize {
  let typeid = TypeId::of::<T>();

  unsafe {
    for (index, id) in REGISTERED_TYPE_IDS.iter().enumerate() {
      if typeid == *id {
        return index
      }
    }

    let result = REGISTERED_TYPE_IDS.len();
    REGISTERED_TYPE_IDS.push(typeid);

    result
  }
}
