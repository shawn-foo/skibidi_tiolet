use bevy::render::primitives::Aabb;
use bevy::{math::vec2, prelude::*};
use bevy::math::bounding::Aabb2d;

use crate::player::HORISONTAL_MOVEMENT_SPEED;
use crate::{console_log, player};
use crate::utils::Collision;
use crate::{tilemap::{resource::CurrentLevelInfo, tile::Tile, TILE_SIZE }, utils::check_collision};

use super::{GROUND_FRICTION, JUMP_STRENGTH};
use super::{components::Player, GRAVITY, PLAYER_SIZE};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_info: Res<CurrentLevelInfo>
){
    commands.spawn(
        (
            SpriteBundle{
                texture: asset_server.load("player.png"),
                sprite: Sprite{
                    custom_size: Option::Some(Vec2::new(PLAYER_SIZE.width, PLAYER_SIZE.height)),
                    ..default()
                },
                transform: Transform::from_xyz(level_info.player_starting_pos.x as f32 * TILE_SIZE.width, level_info.player_starting_pos.y as f32 * TILE_SIZE.height + PLAYER_SIZE.height / 4.0, 2.0),
                ..Default::default()
            },
            Player::new(true, 0.0, 0.0, false),
            Name::new("player")
        )
    );
}

pub fn move_player(
    mut player_query: Query<(& mut Transform, &mut Player), (With<Player>, Without<Tile>)>,
    tile_query: Query<(&Transform), (With<Tile>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    //get player transform and component
    if let Ok((mut player_transform, mut player_info)) = player_query.get_single_mut(){
        //clone translation for calculations
        let mut player_translation = player_transform.translation.clone();
        


        //react to button press
        if keyboard_input.just_pressed(KeyCode::KeyW) && player_info.on_floor != false{
            player_info.vel_y += JUMP_STRENGTH;
            player_info.on_floor = false;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            player_info.vel_x = HORISONTAL_MOVEMENT_SPEED * -1.0;
        } /*  else if keyboard_input.just_pressed(KeyCode::KeyS) {
             player_translation.x -= HORISONTAL_MOVEMENT_SPEED;
        } */ else if keyboard_input.pressed(KeyCode::KeyD) {
            player_info.vel_x = HORISONTAL_MOVEMENT_SPEED;
        }

        //apply gravity
        player_info.vel_y += GRAVITY;

        //apply friction
        if player_info.vel_x < 0.0{
            if player_info.vel_x + GROUND_FRICTION > 0.0{
                player_info.vel_x = 0.0;
            } else {
                player_info.vel_x += GROUND_FRICTION;
            }
        }
        if player_info.vel_x > 0.0{
            if player_info.vel_x - GROUND_FRICTION < 0.0{
                player_info.vel_x = 0.0;
            } else {
                player_info.vel_x -= GROUND_FRICTION;
            }
        }
        console_log!("{}", player_info.vel_x);
        //add velocity
        player_translation.y += player_info.vel_y;
        player_translation.x += player_info.vel_x;
        
        // getting tile trasnform for collisionion
        for (tile_transform) in tile_query.iter(){
            let collision_result = check_collision(Aabb2d::new(player_translation.truncate(), Vec2::new(PLAYER_SIZE.width / 2.0, PLAYER_SIZE.height / 2.0)), Aabb2d::new(tile_transform.translation.truncate(), Vec2::new(TILE_SIZE.width / 2.0, TILE_SIZE.height / 2.0)));
            
            match collision_result{
                None => {},
                Some(collision_side) => {
                    match collision_side{
                        Collision::LEFT => {
                            player_translation.x = tile_transform.translation.x - TILE_SIZE.width / 2.0 - PLAYER_SIZE.width / 2.0;
                            player_info.vel_x = 0.0;
                        },
                        Collision::RIGHT => {
                            player_translation.x = tile_transform.translation.x + TILE_SIZE.width / 2.0 + PLAYER_SIZE.width / 2.0;
                            player_info.vel_x = 0.0;
                        },
                        Collision::TOP => {
                            if player_info.vel_y < 0.0{
                                player_info.vel_y = 0.0;
                                player_info.on_floor = true;
                                player_translation.y = tile_transform.translation.y + TILE_SIZE.height / 2.0 + PLAYER_SIZE.height / 2.0;
                                player_info.jumping = false;                               
                            }


                        },
                        Collision::BOTTOM => {
                            player_info.vel_y = 0.0;
                            player_translation.y = tile_transform.translation.y - TILE_SIZE.height / 2.0 - PLAYER_SIZE.height / 2.0;
                            
                        },

                    }
                }
            }

        }
        

        player_transform.translation = player_translation.clone();
    }
    
}

