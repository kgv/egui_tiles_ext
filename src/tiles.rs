use egui_tiles::{Tile, TileId, Tiles};

/// Extension methods for [`Tiles`]
pub trait TilesExt<T> {
    fn find_pane_by(&mut self, f: impl Fn(&T) -> bool) -> Option<TileId>;

    fn panes<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    fn panes_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;
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

    fn panes<'a>(&self) -> impl Iterator<Item = &T> {
        self.tiles().filter_map(|tile| match tile {
            Tile::Pane(pane) => Some(pane),
            Tile::Container(_) => None,
        })
    }

    fn panes_mut<'a>(&mut self) -> impl Iterator<Item = &mut T> {
        self.tiles_mut().filter_map(|tile| match tile {
            Tile::Pane(pane) => Some(pane),
            Tile::Container(_) => None,
        })
    }
}
