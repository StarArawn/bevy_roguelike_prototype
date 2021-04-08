use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use crate::game::GameState;

pub fn generate_map(mut game_state: ResMut<State<GameState>>, mut map_query: Query<&mut Tilemap>) {
    if *game_state.current() == GameState::Playing {
        return;
    }

    for mut map in map_query.iter_mut() {
        let chunk_width = map.chunk_width() as i32;
        let chunk_height = map.chunk_height() as i32;
        let map_width = map.width().unwrap() as i32;
        let map_height = map.height().unwrap() as i32;

        let actual_width = map_width * chunk_width;
        let actual_height = map_height * chunk_height;

        let half_map_width = map_width / 2;
        let half_map_height = map_height / 2;

        let mut tiles = Vec::new();
        for x in 0..actual_width {
            for y in 0..actual_height {
                let x = x - chunk_width / 2;
                let y = y - chunk_height / 2;
                let tile = Tile {
                    point: (x, y),
                    sprite_index: 18,
                    ..Default::default()
                };

                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        for x in -half_map_width..half_map_width {
            for y in -half_map_height..half_map_height {
                map.spawn_chunk((x, y)).unwrap();
            }
        }

        game_state.set(GameState::Playing).unwrap();
    }

}