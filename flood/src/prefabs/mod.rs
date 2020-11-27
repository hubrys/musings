mod node_prefab;

pub use crate::prefabs::node_prefab::NodePrefabData;
use amethyst::assets::{Prefab, Handle};
use std::collections::HashMap;

pub type NodePrefab = Handle<Prefab<NodePrefabData>>;

// #[derive(Default)]
// pub struct PrefabCache {
//   data: HashMap<String, NodePrefab>
// }
#[derive(Default)]
pub struct PrefabCache {
  prefabs: HashMap<String, NodePrefab>
}

impl PrefabCache {
  pub fn new() -> Self {
    PrefabCache {
      prefabs: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: &str, prefab: NodePrefab) {
    self.prefabs.insert(key.to_string(), prefab);
  }

  pub fn get(&self, key: &str) -> NodePrefab {
    self.prefabs.get(key).unwrap().clone()
  }
}

