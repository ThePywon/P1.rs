use super::component::{Component, get_component_t_id};

pub trait System<A: Component + 'static, B: Component + 'static = A,
  C: Component + 'static = A, D: Component + 'static = A, E: Component + 'static = A> {
  fn match_entity(entity: usize) -> bool {
    let bit_a = 1 << get_component_t_id::<A>();
    let bit_b = 1 << get_component_t_id::<B>();
    let bit_c = 1 << get_component_t_id::<C>();
    let bit_d = 1 << get_component_t_id::<D>();
    let bit_e = 1 << get_component_t_id::<E>();
    let bitfield = bit_a & bit_b & bit_c & bit_d & bit_e;
    let component_mask = entity;

    bitfield & component_mask == bitfield
  }
}
