use egui_tiles::{Container, Tile, Tiles};

/// [`Container`] extension methods
pub trait ContainerExt {
    fn find_child_pane<'a, T>(&'a self, tiles: &'a Tiles<T>) -> Option<&'a T>;

    // fn active_panes<'a, T>(&'a self, tiles: &'a Tiles<T>, f: impl Fn(&T));
}

impl ContainerExt for Container {
    fn find_child_pane<'a, T>(&'a self, tiles: &'a Tiles<T>) -> Option<&'a T> {
        self.children().find_map(|child| match tiles.get(*child)? {
            Tile::Container(container) => container.find_child_pane(tiles),
            Tile::Pane(pane) => Some(pane),
        })
    }

    // fn active_panes<'a, T>(&'a self, tiles: &'a Tiles<T>, f: impl Fn(&T)) {
    //     for child in self.active_children() {
    //         match tiles.get(*child).unwrap() {
    //             Tile::Container(container) => container.active_panes(tiles, &f),
    //             Tile::Pane(pane) => f(pane),
    //         }
    //     }
    // }
}
