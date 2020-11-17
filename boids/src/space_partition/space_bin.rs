use amethyst::core::math::Vector2;
use crate::components::Movement;

#[derive(Default)]
pub struct TiledSpacePointer {
  id: u32,
  tile_index: usize
}

struct NodeElement<T>(u32, T);

struct Node<T> {
  elems: Vec<NodeElement<T>>
}


#[derive(Default)]
pub struct TiledSpace<T> {
  next_id: u32,
  width: usize,
  height: usize,
  space_width: f32,
  space_height: f32,
  tile_height: f32,
  tile_width: f32,
  grid: Vec<Node<T>>
}

// impl Default for TiledSpace<&Movement> {
//   fn default() -> Self {
//     unimplemented!()
//   }
// }

impl<T> TiledSpace<T> {
  pub fn new(space_width: f32, space_height: f32, width: usize, height: usize) -> TiledSpace<T> {
    let grid_size = height * width;
    let mut grid = Vec::with_capacity(grid_size as usize);
    for _ in 0..grid_size {
      grid.push(Node {
        elems: Vec::with_capacity(10)
      });
    }

    TiledSpace {
      next_id: 1,
      width,
      height,
      space_width,
      space_height,
      tile_width: space_width / width as f32,
      tile_height: space_height / height as f32,
      grid
    }
  }

  fn next_id(&mut self) -> u32 {
    let id = self.next_id;
    self.next_id += 1;
    id
  }

  pub fn add_elem(&mut self, coords: Vector2<f32>, elem: T) -> TiledSpacePointer {
    let tile_x = (coords.x / self.tile_width).floor() as usize;
    let tile_y = (coords.y / self.tile_height).floor() as usize;
    let tile_index = tile_y * self.width + tile_x;
    let new_id = self.next_id();
    if let Some(tile) = self.grid.get_mut(tile_index) {
      tile.elems.push(NodeElement(new_id, elem));
      return TiledSpacePointer {
        id: new_id,
        tile_index,
      };
    }
    panic!("WE SHOULDNT BE HERE");
  }

  pub fn move_elem(&mut self, pointer: &mut TiledSpacePointer, new_coords: Vector2<f32>) {
    let tile_x = (new_coords.x / self.tile_width).floor() as usize;
    let tile_y = (new_coords.y / self.tile_height).floor() as usize;
    let tile_index = tile_y * self.width + tile_x;
    if tile_index == pointer.tile_index {
      // Since the indexes are the same, we don't have to do anything.
      // Already in the correct tile
      return;
    }

    let mut old_tile = self.grid.get_mut(pointer.tile_index).unwrap();
    for i in 0..old_tile.elems.len() {
      if old_tile.elems[i].0 == pointer.id {
        // We don't care about the order of these arrays, just that the element
        // is removed
        let elem = old_tile.elems.swap_remove(i);
        if let Some(new_tile) = self.grid.get_mut(tile_index) {
          new_tile.elems.push(elem);
          pointer.tile_index = tile_index;
        } else {
          panic!("WE SHOULDNT BE HERE: TWO");
        }
        break;
      }
    }
  }

  pub fn get_elems(&self, boundary: [f32; 4]) -> Iter<T> {
    Iter {
      start_x: (boundary[0] / self.tile_width).min(0.0).max(self.space_width) as usize,
      start_y: (boundary[1] /self.tile_height).min(0.0).max(self.space_height) as usize,
      end_x: (boundary[2] / self.tile_width).min(0.0).max(self.space_width) as usize,
      end_y: (boundary[3] / self.tile_height).min(0.0).max(self.space_height) as usize,
      grid_width: self.width,
      grid_index: 0,
      node_iter: None,
      grid: &self.grid
    }
  }
}

pub struct Iter<'a, T> {
  start_x: usize,
  start_y: usize,
  end_x: usize,
  end_y: usize,
  grid_width: usize,
  grid_index: usize,
  node_iter: Option<std::slice::Iter<'a, NodeElement<T>>>,
  grid: &'a Vec<Node<T>>
}

impl<'a, T> Iter<'a, T> {
  fn init(&mut self) {
    self.grid_index = 0;
    self.set_node_iter();
  }

  fn set_node_iter(&mut self) {
    let search_width  = self.end_x - self.start_x;
    let real_x = (self.grid_index % search_width) + self.start_x;
    let real_y = (self.grid_index / search_width) + self.start_y;
    self.node_iter = {
      if real_y > self.end_y {
        None
      } else {
        let real_index = real_y * self.grid_width + real_x;
        Some(self.grid[real_index].elems.iter())
      }
    }
  }
}

impl<'a, T: Copy> Iterator for Iter<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.node_iter {
      None => None,
      Some(iter) => {
        let maybe_value = iter.next();
        if let None = maybe_value {
          // go to next node
          self.grid_index += 1;
          return self.next()
        }
        maybe_value.map(|elem| {
          elem.1
        })
      }
    }
  }
}

#[cfg(test)]
mod tests {
  // #[test]
  // fn
}