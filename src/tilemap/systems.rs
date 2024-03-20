use bevy::{prelude::*, render::view::window, transform::commands};


use super::{resource::{CurrentLevelInfo, TilemapResource}, tile::{Tile, TilePosition, TileType}, PADDING_BEFORE_DESPAWN, TILE_SIZE};

// pub fn update_screen(
//     mut level_storage: Res<LevelStorage>,
//     camera_query: Query<(&mut Transform), (With<Camera>)>,
//     window_query: Query<(&Window)>
// ){
//     if let Ok(camera_transform) = camera_query.get_single(){
//         if let Ok(window) = window_query.get_single(){
//             let starting = Vec2::new(camera_transform.translation.x - window.width() / 2.0, camera_transform.translation.y - window.height() / 2.0);
//             let 
//         }
        
//     }
// }

pub fn spawn_tilemap(
    mut commands: Commands,
    window_query: Query<&Window>,
    level_info: Res<CurrentLevelInfo>,
    asset_server: Res<AssetServer>
){
    // if let Ok(window) = window_query.get_single(){
    //     tile_resource.number_in_screen.height = ((( window.height()) / tile_resource.tile_size.height) as i32) / as f32;
    //     tile_resource.number_in_screen.width = ((( window.width() + PADDING_BEFORE_DESPAWN) / tile_resource.tile_size.height) as i32) as f32;

    // }
    for (y , x_vector) in level_info.level.iter().enumerate(){
        for (x, value) in x_vector.iter().enumerate(){
            if value.clone() == 1{
                commands.spawn(
                    (
                        SpriteBundle{
                            texture: asset_server.load("dirt.png"),
                            transform: Transform::from_xyz((x * TILE_SIZE.width as usize) as f32, (y * TILE_SIZE.height as usize) as f32, 0.0),
                            sprite: Sprite{
                                custom_size: Option::Some(Vec2::new(TILE_SIZE.width, TILE_SIZE.height)),
                                ..default()
                            },
                            ..Default::default()
                        }, 
                        Tile::new(TileType::WALL, TilePosition::new(x as i32, y as i32))
                    )
                );
            }
        }
    }
}