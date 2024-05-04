use bevy::a11y::accesskit::Rect;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use log::{warn, error, debug};


use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, PIXEL_SCALE};

use super::resources::*;


pub fn game_plugin(app: &mut App) {
    app
    .add_systems(OnEnter(GameState::Loading), load_room)
    .add_systems(OnEnter(GameState::Running), create_game_objects)

    .add_systems(Update,player_movement.run_if(in_state(GameState::Running)))
    .add_systems(Update, move_camera.run_if(in_state(GameState::Running)));
}

//Component Used to tag the player
#[derive(Component)]
struct Player;


//Component Used to tag a Static Object that does nothing
#[derive(Component)]
struct StaticObject;


fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&mut Transform, With<Player>>
){
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
){
    let room_texture = asset_server.load("Textures/Rooms/Room-concept-01.png");
    #[cfg(target_os = "Windows")]{
        room_texture = asset_server.load("Textures\\Rooms\\Room-concept-01.png");
    }

    commands.spawn(
        (SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIXEL_SCALE * 11.0, PIXEL_SCALE * 8.0)),
                .. default()
            },
            texture: room_texture,
            .. default()
        },
        StaticObject
    )
    );

    let player_texture = asset_server.load("Textures/Player/Player-Singlet.png");
    #[cfg(target_os ="Windows")]{
        let player_texture = asset_server.load("Textures\\Player\\Player-Singlet.png");
    }

    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                flip_x: false,
                anchor: Anchor::BottomLeft,
                .. default()
            },
            texture: player_texture,
            transform: Transform { scale: Vec3{ x: 30.0, y: 96.0, z: 0.0 }, ..Default::default()},
            .. default()
        },
        Player,
    ));

    
}

fn player_movement(
    mut players: Query<(&mut Transform, &Player, &mut Sprite)>,
    mut colliders: Query<&Collider>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, _, mut sprite) in &mut players {
        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 150.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 150.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 150.0 * time.delta_seconds();
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 150.0 * time.delta_seconds();
            sprite.flip_x = true;
        }
        
        // println!("Transform: x:{} , y:{}",transform.translation.x, transform.translation.y);

    }

    //CODE FOR COLLIDERS::

    for (mut player_transform, _, _) in &mut players{

        //create a rect containing the current location of the player
        
        let px_scale: f64 = player_transform.scale.x as f64;
        let p_left: f64 = player_transform.translation.x as f64;
        let p_right: f64 = p_left + px_scale;

        
        let p_bot: f64 = player_transform.translation.y as f64;
        let py_scale: f64 = player_transform.scale.y as f64;
        let p_top: f64 = p_bot + py_scale;

        let p_rect = Rect::new(p_left, p_bot, p_right, p_top);


        for collider in &mut colliders{
            //we need to check if the player is inside this collider, if so we need to push them outside of it

            //create a rect to test against the player rect
            let cx_scale: f64 = collider.transform.scale.x as f64;
            let c_left: f64 = collider.transform.translation.x as f64;
            let c_right: f64 = collider.transform.translation.x as f64 + cx_scale;

            let c_top: f64 = collider.transform.translation.y as f64;
            let cy_scale: f64 = collider.transform.scale.y as f64;
            let c_bot: f64 = collider.transform.translation.y as f64 - cy_scale;

            let c_rect = Rect::new(c_left, c_bot, c_right, c_top);

            debug!("c_rect: {c_rect:?}");
            debug!("p_rect: {p_rect:?}");
                        
            if p_rect.intersect(c_rect).area() != 0.0{
                
                if collider.style == ColliderType::RIGID{
                    debug!("INTERSECTION DETECTED!");
                    let intersection = p_rect.intersect(c_rect);

                    if intersection.width() < intersection.height() {
                        
                        if p_rect.min_x() < c_rect.min_x() {
                            player_transform.translation.x -= intersection.width() as f32;
                        } else if p_rect.max_x() > c_rect.max_x(){
                            player_transform.translation.x += intersection.width() as f32;
                        }
                    } else if intersection.width() > intersection.height(){
                        if p_rect.min_y() < c_rect.min_y() {
                            player_transform.translation.y -= intersection.height() as f32;
                        } else if p_rect.max_y() > c_rect.max_y(){
                            player_transform.translation.y += intersection.height() as f32;
                        }
                    }
                }
            }
    
        }
    }

}
    

#[derive(Component, PartialEq, Debug)]
enum ColliderType{
    RIGID,
    INTERACTABLE,
    TRIGGER
}

#[derive(Component)]
struct Collider {
    transform: Transform,
    style: ColliderType,
}

//Helper function for loading files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn load_room(
    mut commands: Commands,
    current_room: Res<CurrentRoom>,
    mut game_state: ResMut<NextState<GameState>>,

    in_debug: Res<DebugMode>
){
    let level = current_room.0;
    let room = current_room.1;
    let var = current_room.2;


    let file_name = format!("assets/Maps/Room-col{}{}{}.svg", level, room, var).to_string();
    #[cfg(target_os = "Windows")]{
    file_name = format!("assets\\Maps\\Room-col{}{}{}.svg", level, room, var).to_string();
    }

    warn!("{}", file_name);

    let mut i =0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //seems like for each line of this file we can create a new collider object
            //lets parse out the important info first
            i += 1;
            if i <=2 {
                continue;
            }

            if line.contains("svg"){
                println!("End of file: {}", line);
                break;
            }

            //create important variables we will use to create our collider
            let mut x:i16 = 0;
            let mut y:i16 = 0;
            let mut w:i16 = 0;
            let mut h:i16 = 0;
            let mut col: &str = "";

            let pretty_line = line.trim();
            let parts = pretty_line.split("\"").collect::<Vec<_>>();

            let mut ints = Vec::<i16>::new();

            let mut count = 0;
            for part in parts{
                count+= 1;
                if (count-1)%2 ==0{
                    continue;
                }

                


                match part.parse::<i32>(){
                    Ok(_) =>{
                        println!("Parsed: {}", part.parse::<i16>().unwrap());
                        ints.push(part.parse::<i16>().unwrap());
                    },

                    Err(_) =>{
                        if part.contains("#"){
                            println!("Found Color: {}", part);
                            col = part;

                        }else{
                            warn!("Could not parse int from part: {}", part)
                        }
                    }
                }
                
            }

            count = 0;
            for item in ints{
                
                match count % 4 {
                    0 =>{
                        x= item
                    },
                    
                    1 => {
                        y= item
                    },

                    2=> {
                        w= item
                    },

                    3=> {
                        h= item
                    },

                    _=>{
                        warn!("This should never print")
                    }
                }
                count+=1;
            }

            let st = match col{
                "#000000" =>{
                    ColliderType::RIGID
                },

                "#00FF00" =>{
                    ColliderType::TRIGGER
                },

                _ =>{
                    ColliderType::INTERACTABLE
                }


            };

            col = col.split("#").collect::<Vec<_>>()[1];
            println!("COLOR FOUND: {}", col);

            println!("Creating Collider with x:{} y:{} w:{} h:{} of type:{:?}", ((x*48) as f32 - SCREEN_WIDTH/2.0), ((SCREEN_HEIGHT / 2.0) + (y*48) as f32), w*96, h*96, st);

                commands.spawn(
                    (Collider {
                        // transform: Rect::new((x*96).into(), (y*96).into(), ((x+w)*96).into(), ((y+h)*96).into()),
                        transform: Transform { 
                            translation: Vec3::new((x as f32 * PIXEL_SCALE)  - SCREEN_WIDTH/2.0 , (SCREEN_HEIGHT / 2.0) - (y as f32 * PIXEL_SCALE), in_debug.0 as i32 as f32),
                            scale: Vec3::new(w as f32 * PIXEL_SCALE, h as f32 * PIXEL_SCALE, 0.0),
                            .. default()
                        },
                        style: st
                    },
                    SpriteBundle{
                        transform: Transform { 
                            translation: Vec3::new((x as f32 * PIXEL_SCALE)  - SCREEN_WIDTH/2.0 , (SCREEN_HEIGHT / 2.0) - ((y as f32 * PIXEL_SCALE)+ PIXEL_SCALE), in_debug.0 as i32 as f32),
                            scale: Vec3::new(w as f32 * PIXEL_SCALE, h as f32 * PIXEL_SCALE, 0.0),
                            
                            .. default()
                        },
                        sprite: Sprite{
                            color: Color::hex(col).unwrap(),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        .. default()
                    }
                )
                );

        }
    }

    //finished loading room change InGameState to Running
    game_state.set(GameState::Running);
}