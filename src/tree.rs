use egui_tiles::{ContainerKind, Tile, Tree};

pub const TABS: usize = ContainerKind::Tabs as _;
pub const HORIZONTAL: usize = ContainerKind::Horizontal as _;
pub const VERTICAL: usize = ContainerKind::Vertical as _;
pub const GRID: usize = ContainerKind::Grid as _;

/// [`Tree`] extension methods
pub trait TreeExt<T> {
    fn insert_pane<const CONTAINER_KIND: usize>(&mut self, pane: T);
}

impl<T> TreeExt<T> for Tree<T> {
    fn insert_pane<const CONTAINER_KIND: usize>(&mut self, pane: T) {
        let child = self.tiles.insert_pane(pane);
        if let Some(root) = self.root {
            if let Some(tile) = self.tiles.get_mut(root) {
                if let Tile::Container(container) = tile {
                    container.add_child(child);
                } else {
                    self.root = match CONTAINER_KIND {
                        TABS => Some(self.tiles.insert_tab_tile(vec![root, child])),
                        HORIZONTAL => Some(self.tiles.insert_horizontal_tile(vec![root, child])),
                        VERTICAL => Some(self.tiles.insert_vertical_tile(vec![root, child])),
                        GRID => Some(self.tiles.insert_grid_tile(vec![root, child])),
                        _ => unreachable!(),
                    };
                }
            }
        } else {
            self.root = Some(child)
        }
    }
}
