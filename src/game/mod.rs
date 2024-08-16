use bevy::a11y::accesskit::Rect;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::{debug, warn};

use crate::{IS_IN_WINDOWS, PIXEL_SCALE};

use super::resources::*;

mod rooms;


pub fn game_plugin(app: &mut App) {
    app
    
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_plugins(rooms::room_plugin)
        
        .add_systems(OnEnter(GameState::LevelLoading), create_game_objects)

        .add_systems(FixedUpdate, (
            player_movement,
            collision_detection,
            move_camera,
        ).run_if(in_state(GameState::Running)));
}

//Component Used to tag the player and give it velocity
#[derive(Component)]
struct Player {
    vel_x: f32,
    vel_y: f32,

    hitbox: Rect,
}

//Component Used to tag a Static Object that does nothing
#[derive(Component)]
struct StaticObject;

fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&mut Transform, With<Player>>,
) {
    //move camera to be centered on player
    //player's sprite is anchored to the bottom left
    for player_transform in &mut player.iter() {
        let mut camera_transform = camera.single_mut();
        camera_transform.translation = player_transform.translation;
    }
}

fn create_game_objects(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    
    let tex;

    if IS_IN_WINDOWS {
        tex = asset_server.load("textures\\player\\player-singlet.png");
    }else {
        tex = asset_server.load("textures/player/player-singlet.png");
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.625, 2.0)),
                flip_x: false,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: tex,
            transform: Transform {
                scale: Vec3 {
                    x: PIXEL_SCALE,
                    y: PIXEL_SCALE,
                    z: 1.0,
                },
                ..Default::default()
            },
            ..default()
        },
        Player {
            vel_x: 0.0,
            vel_y: 0.0,

            hitbox: Rect::new(0.0, 0.0, PIXEL_SCALE as f64 * 0.625, PIXEL_SCALE as f64 * 0.125),
        },
    ));
}

fn collision_detection(
    mut player: Query<(&mut Transform, &mut Player)>,
    colliders: Query<&Collider, Without<Player>>,
) {
    for (mut player_transform, _) in &mut player {
        //create a rect containing the current location of the player

        let px_scale: f64 = (PIXEL_SCALE * 0.625) as f64;
        let p_left: f64 = player_transform.translation.x as f64;
        let p_right: f64 = p_left + px_scale;

        let p_bot: f64 = player_transform.translation.y as f64;
        let py_scale: f64 = player_transform.scale.y as f64 * 0.2;
        let p_top: f64 = p_bot + py_scale;

        let p_rect = Rect::new(p_left, p_bot, p_right, p_top);

        for collider in &colliders {
            //we need to check if the player is inside this collider, if so we need to push them outside of it

            //create a rect to test against the player rect
            let cx_scale: f64 = collider.transform.scale.x as f64; //size of collider
            let c_left: f64 = collider.transform.translation.x as f64; //left side of collider on x
            let c_right: f64 = collider.transform.translation.x as f64 + cx_scale; //right side of collider calculated from left side and size

            let c_top: f64 = collider.transform.translation.y as f64; //top of collider
            let cy_scale: f64 = collider.transform.scale.y as f64; //size of collider on y
            let c_bot: f64 = collider.transform.translation.y as f64 - cy_scale; //bottom of collider calculated from top and size

            let c_rect = Rect::new(c_left, c_bot, c_right, c_top);

            debug!("c_rect: {c_rect:?}");
            debug!("p_rect: {p_rect:?}");

            if p_rect.intersect(c_rect).area() != 0.0 {
                if collider.style == ColliderType::RIGID {
                    debug!("INTERSECTION DETECTED!");
                    let intersection = p_rect.intersect(c_rect);

                    if intersection.width() < intersection.height() {
                        if p_rect.min_x() < c_rect.min_x() {
                            player_transform.translation.x =
                                c_rect.min_x() as f32 - px_scale as f32;
                        } else if p_rect.max_x() > c_rect.max_x() {
                            player_transform.translation.x = c_rect.max_x() as f32;
                        }
                    } else if intersection.width() > intersection.height() {
                        if p_rect.min_y() < c_rect.min_y() {
                            player_transform.translation.y =
                                c_rect.min_y() as f32 - py_scale as f32;
                        } else if p_rect.max_y() > c_rect.max_y() {
                            player_transform.translation.y = c_rect.max_y() as f32;
                        }
                    }
                }
            }
        }
    }
}

fn player_movement(
    mut players: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player, mut sprite) in &mut players {
        if input.pressed(KeyCode::ArrowUp) && !input.pressed(KeyCode::ArrowDown) {
            player.vel_y = 120.0;
        }
        if input.pressed(KeyCode::ArrowDown) && !input.pressed(KeyCode::ArrowUp) {
            player.vel_y = -120.0;
        }
        if input.pressed(KeyCode::ArrowRight) && !input.pressed(KeyCode::ArrowLeft) {
            player.vel_x = 150.0;
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::ArrowLeft) && !input.pressed(KeyCode::ArrowRight) {
            player.vel_x = -150.0;
            sprite.flip_x = true;
        }

        //apply velocity
        transform.translation.y += player.vel_y * time.delta_seconds();
        transform.translation.x += player.vel_x * time.delta_seconds();

        //apply friction
        player.vel_y = player.vel_y * 0.99 as i32 as f32;
        player.vel_x = player.vel_x * 0.99 as i32 as f32;
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
enum ColliderType {
    RIGID,
    Interactable,
    ChangeRoom,
}

#[derive(Component, Clone, Copy)]
struct Collider {
    transform: Transform,
    style: ColliderType,
}


//Helper function for loading files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    warn!("Reading file: {:?}", filename.as_ref());

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}