use egui_tiles::{Tile, TileId, Tiles};

/// Extension methods for [`Tiles`]
pub trait TilesExt<T> {
    fn find_pane_by(&mut self, f: impl Fn(&T) -> bool) -> Option<TileId>;
}

impl<T> TilesExt<T> for Tiles<T> {
    fn find_pane_by(&mut self, f: impl Fn(&T) -> bool) -> Option<TileId> {
        self.iter()
            .find(|(_, tile)| {
                if let Tile::Pane(pane) = *tile {
                    f(pane)
                } else {
                    false
                }
            })
            .map(|(tile_id, _)| *tile_id)
    }
}
