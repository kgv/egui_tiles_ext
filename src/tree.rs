use egui_tiles::{Tile, Tree};

/// [`Tree`] extension methods
pub trait TreeExt<T> {
    fn insert_pane(&mut self, pane: T);
}

impl<T> TreeExt<T> for Tree<T> {
    fn insert_pane(&mut self, pane: T) {
        let child = self.tiles.insert_pane(pane);
        if let Some(root) = self.root {
            if let Some(tile) = self.tiles.get_mut(root) {
                if let Tile::Container(container) = tile {
                    container.add_child(child);
                } else {
                    self.root = Some(self.tiles.insert_vertical_tile(vec![root, child]));
                }
            }
        } else {
            self.root = Some(child)
        }
    }
}
