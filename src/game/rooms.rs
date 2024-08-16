use std::{fs, io};

use bevy::{a11y::accesskit::Rect, transform::commands};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use log::warn;

use crate::{game::{read_lines, ColliderType}, IS_IN_WINDOWS, PIXEL_SCALE};

use super::{Collider, DebugMode, GameState, Player, Shadow, };

use crate::resources::*;


pub fn room_plugin(app: &mut App){
    app
        .add_systems(OnEnter(GameState::LevelLoading), load_level_room_data)

        .add_systems(OnEnter(GameState::Loading), (
            spawn_colliders,
            display_rooms,
        ).chain().run_if(in_state(GameState::Loading)))

        .add_systems(Update, (
            room_status,
            despawn_rooms,
        ).run_if(in_state(GameState::Running)));
}


#[derive(Component, Clone, Debug)]
struct Room {
    identifier: String,

    location: Transform,
    area: Rect,

    backdrop_path: String,
    decoration_path: String,
    foreground_path: String,
    
    colliders: Vec<Collider>,

    active: bool,
    lifetime: u32,

}

#[derive(Component)]
struct RoomId(String);


fn display_rooms(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut rooms: Query<&mut Room>,

    mut game_state: ResMut<NextState<GameState>>,
) {
    for room in &mut rooms {
        //check if the player's hitbox intersects with the room's area
        
        if room.active {
            //this is an active room that should be displayed
            println!("Attempting to display room: {:?}", room.backdrop_path);
            let backdrop = asset_server.load(room.backdrop_path.clone());
            let decoration = asset_server.load(room.decoration_path.clone());
            let foreground = asset_server.load(room.foreground_path.clone());
            let normalized_z_index = (1.0 / (1.0 + f64::exp(-0.1 * room.location.translation.y as f64))) as f32;

            
            //Background
            //Z-Index ranges from -1 to 0
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    texture: backdrop,
                    transform: Transform {
                        translation: Vec3::new(
                            room.location.translation.x,
                            room.location.translation.y,
                            0.0 + normalized_z_index,
                        ),
                        scale: room.location.scale,
                        ..default()
                    },
                    ..default()
                },
                RoomId(room.identifier.clone())
            ));

            //Decoration
            //Z-Index ranges from 0 to 1
            commands.spawn(
                (
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        texture: decoration,
                        transform: Transform {
                            translation: Vec3::new(
                                room.location.translation.x,
                                room.location.translation.y,
                                1.0 + normalized_z_index,
                            ),
                            
                            scale: room.location.scale,
                            .. default()
                        },
                        ..default()
                    }, 
                    RoomId(room.identifier.clone())
                )
            );

            //Foreground
            //Z-Index ranges from 1 to 2
            commands.spawn(
                (
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        texture: foreground,
                        transform: Transform {
                            translation: Vec3::new(
                                room.location.translation.x,
                                room.location.translation.y,
                                3.0 + normalized_z_index,
                            ),
                                
                            scale: room.location.scale,
                            .. default()
                        },
                        ..default()
                    }, 
                    RoomId(room.identifier.clone())
                )
            );
        }//end of if active
    }//end of for loop
    game_state.set(GameState::Running);
}


///this function will spawn all colliders for all active rooms by making a bevy entity for each collider
/// This function is scheduled by bevy and will run in the loading state
fn spawn_colliders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    rooms: Query<&Room>,
    in_debug: Res<DebugMode>,
) {


    for room in &rooms {
        if room.active {
            for collider in &room.colliders {
                commands.spawn((
                    *collider,
                    RoomId(room.identifier.clone())
                ));
    
                if in_debug.0 {
                    // info!("A collider was created at: {:?}", collider.transform);

                    let tex;

                    if IS_IN_WINDOWS{
                        tex = asset_server.load("Textures\\Rooms\\cldr.png");
                    }else{
                        tex = asset_server.load("Textures/Rooms/cldr.png");
                    }

                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                anchor: Anchor::TopLeft,
                                ..default()
                            },
                            transform: collider.transform,
                            texture: tex,
                            ..default()
                        },
                        RoomId(room.identifier.clone())
                    ));
                }
            }
        }
    }
}



fn room_status(
    mut rooms: Query<&mut Room>,
    players: Query<(&Transform, &Player), Without<Shadow>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    'rooms: for mut room in &mut rooms {
        //set room to active it the room's rect intersects with the player's rect
        let mut needs_reload = false;
        
        for (player_transform, _) in &players {
            let px_scale: f64 = PIXEL_SCALE as f64 * 0.625;
            let p_left: f64 = player_transform.translation.x as f64;
            let p_right: f64 = p_left + px_scale;

            let p_bot: f64 = player_transform.translation.y as f64;
            let py_scale: f64 = player_transform.scale.y as f64;
            let p_top: f64 = p_bot + py_scale;

            let p_rect = Rect::new(p_left, p_bot, p_right, p_top);

            if room.area.intersect(p_rect).area() != 0.0 {
                if room.active {
                    //player is in this room and it is already active
                    continue 'rooms;
                } else {
                    //player is intersecting an inactive room, we now need to re-load rooms and display
                    room.active = true;
                    needs_reload = true;
                }
            }else{
                //player is not intersecting this room
                if room.active {
                    //we are no longer intersecting an active room that should be de-activated
                    warn!("Player is not intersecting room that is still active: {:?}", room.location);
                    room.active= false;
                    room.lifetime = 128;
                }
            }
        }
        if needs_reload {
            game_state.set(GameState::Loading);
        }

    }
}


fn despawn_rooms(
    mut rooms: Query<&mut Room>,
    mut room_objects: Query<(Entity, &RoomId)>,
    mut commands: Commands,
) {
    //despawn any room that is no longer active
    for mut room in &mut rooms{

        if room.lifetime > 0 {
            room.lifetime -= 1;

            if room.lifetime == 0 {
                warn!("Despawning room: {:?}", room.location);

                for room_object in &mut room_objects {
                    if room_object.1.0 == room.backdrop_path {
                        //despawn this entity and all of its components
                        commands.entity(room_object.0).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn read_directory(path: &String) -> Result<fs::ReadDir, io::Error> {
    let paths = fs::read_dir(path);

    match paths {
        Ok(_) => {
            paths
        }
        Err(_) => {
            error!("Failed to find / read a directory");
            paths
        }
    }
} 

///This function will load the room data from the rooms folder
/// This function is scheduled by bevy and will run in the loadinglevel state
fn load_level_room_data(
    mut commands: Commands,
    in_debug: Res<DebugMode>,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,

    mut game_state: ResMut<NextState<GameState>>,
) {
    //for each file in the rooms folder load the room data
    let rooms_path: String = format!("Assets/textures/rooms/L{}", current_level.0);
    println!("Looking for rooms in: {}", rooms_path);

    let paths = read_directory(&rooms_path);
    

    for item in paths.unwrap() {

        match item {
            Ok(item) => {

                match item.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            warn!("Found directory in rooms folder: {}", item.path().display());
                            let new_room = create_room(item.path().display().to_string(),&asset_server);
                            commands.spawn(
                                new_room.clone()
                            );

                            if in_debug.0 {

                                let tex: Handle<Image>;
                                if IS_IN_WINDOWS{
                                    tex = asset_server.load("textures\\rooms\\room_border.png");
                                }else{
                                    tex = asset_server.load("textures/rooms/room_border.png");
                                }
                                commands.spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(1.0, 1.0)),
                                            anchor: Anchor::BottomLeft,
                                            ..default()
                                        },
                                        transform: Transform {
                                            translation: new_room.location.translation,
                                            scale: Vec3::new(new_room.area.width() as f32, new_room.area.height() as f32, 0.0),
                                            ..default()
                                        },
                                        texture: tex,
                                        ..default()
                                    },
                                    RoomId(item.path().display().to_string())
                                ));
                            }



                        }else {
                            warn!("Found file in rooms folder, this may have been a mistake: {}", item.path().display());
                        }
                    }
                    Err(_) => {
                        warn!("Could not read file type in rooms folder");
                    }
                }

            },
            Err(_) => {
                warn!("Could not read item in rooms folder");
            }
        }

    game_state.set(GameState::Loading);
}}


fn create_room(directory_path: String, asset_server: &Res<AssetServer>) -> Room {
    warn!("Creating Room from directory: {}", directory_path);


    let location_info: Vec<&str> = directory_path.split("_").collect();
    info!("Attempting to identify location in path: {:?}", location_info);

    let location = Transform {
        translation: Vec3::new(
            location_info[1].parse::<f32>().unwrap() * PIXEL_SCALE,
            location_info[2].parse::<f32>().unwrap() * PIXEL_SCALE,
            -1.0,
        ),
        scale: Vec3::new(6.0, 6.0, 0.0),
        ..default()
    };
    info!("Creating room at location:{:?}", location.translation);

    let mut room = Room {
        identifier: directory_path.clone(),
        location: location,
        area: Rect{..default()},
        
        backdrop_path: "".to_string(),
        decoration_path: "".to_string(),
        foreground_path: "".to_string(),
        colliders: Vec::<Collider>::new(),

        active: false,
        lifetime: 0,
    };

    let room_items = read_directory(&directory_path).unwrap();
    for item in room_items {
        match item {
            Ok(item) => {
                let mut item_name = item.path().display().to_string();
                item_name = item_name.replace("Assets/", "");

                warn!("Found item: {} in room folder: {}", &directory_path, item_name);

                if item_name.contains("back") {
                    room.backdrop_path = item_name.clone();
                } else if item_name.contains("fore") {
                    room.foreground_path = item_name.clone();
                } else if item_name.contains("deco") {
                    room.decoration_path = item_name.clone();
                } else if item_name.contains("cldr") {
                    room.area = get_area(&item_name, &room.location);

                    let colliders = load_colliders(item_name.clone(), &location, &room.area);
                    room.colliders = colliders.clone();
                }
            }
            Err(_) => {
                warn!("Could not read item in room folder");
            }
        }
    }
    return room;
}


///This function will parse the collider file and return a vector of colliders
/// This function is NOT scheduled by bevy
fn load_colliders(backdrop_path: String, room_location: &Transform, room_area: &Rect) -> Vec<Collider> {

    let mut collider_path: String = backdrop_path.replace("back", "cldr");
    collider_path = collider_path.replace(".png", ".svg");
    
    if IS_IN_WINDOWS{
        collider_path = collider_path.replace("textures", "Assets\\textures");
    }else{
        collider_path = collider_path.replace("textures", "Assets/textures");
    }


    warn!("parsing level colliders from: {}", collider_path);

    let mut colliders = Vec::<Collider>::new();
    let mut i = 0;

    if let Ok(lines) = read_lines(collider_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //seems like for each line of this file we can create a new collider object
            //lets parse out the important info first

            //line zero is not useful to us so we will increment i at the beginning of the loop to skip index 0
            i += 1;

            //the rest logic is not useful for the first 2 lines so we will skip them after this.
            if i <= 2 {
                continue;
            }

            //stop this loop if we find the SVG end tag
            if line.contains("/svg") {
                // println!("End of file: {}", line);
                break;
            }

            //create important variables we will use to create our collider
            let mut x: i16 = 0;
            let mut y: i16 = 0;
            let mut w: i16 = 0;
            let mut h: i16 = 0;
            let mut col: &str = "";

            let pretty_line = line.trim();
            let parts = pretty_line.split("\"").collect::<Vec<_>>();

            let mut ints = Vec::<i16>::new();

            let mut count = 0;
            for part in parts {
                count += 1;
                if (count - 1) % 2 == 0 {
                    continue;
                }

                match part.parse::<i32>() {
                    Ok(_) => {
                        // println!("Parsed: {}", part.parse::<i16>().unwrap());
                        ints.push(part.parse::<i16>().unwrap());
                    }

                    Err(_) => {
                        if part.contains("#") {
                            // println!("Found Color: {}", part);
                            col = part;
                        } else {
                            warn!("Could not parse int from part: {}", part);
                        }
                    }
                }
            }

            count = 0;
            for item in ints {
                match count % 4 {
                    0 => x = item,

                    1 => y = item,

                    2 => w = item,

                    3 => h = item,

                    _ => {
                        warn!("This should never print")
                    }
                }
                count += 1;
            }

            //THIS ACTS AS A KEY TO WHICH COLORS YOU SHOULD BE MAKING YOUR COLLIDERS TO GET THE DESIRED COLLIDERTYPE
            let st = match col {
                "#000000" => ColliderType::RIGID,

                "#00FF00" => ColliderType::ChangeRoom,

                _ => ColliderType::Interactable,
            };

            col = col.split("#").collect::<Vec<_>>()[1];
            // println!("COLOR FOUND: {}", col);

            // println!("Creating Collider with x:{} y:{} w:{} h:{} of type:{:?}", ((x as f32*PIXEL_SCALE)  - SCREEN_WIDTH/2.0), ((SCREEN_HEIGHT / 2.0) + (y as f32*PIXEL_SCALE) as f32), w as f32*PIXEL_SCALE, h as f32*PIXEL_SCALE, st);
            colliders.push(Collider {
                // transform: Rect::new((x*96).into(), (y*96).into(), ((x+w)*96).into(), ((y+h)*96).into()),

                //(x as f32 * PIXEL_SCALE) + PIXEL_SCALE * 2.0 - (),
                transform: Transform {
                    translation: Vec3::new(
                        room_location.translation.x + x as f32 * PIXEL_SCALE,
                        (room_area.height() as f32 - (1.0 * PIXEL_SCALE)) as f32+ room_location.translation.y - (y as f32 * PIXEL_SCALE),
                        5.0
                    ),
                    scale: Vec3::new(w as f32 * PIXEL_SCALE, h as f32 * PIXEL_SCALE, 0.0),
                    ..default()
                },
                style: st,
            });
        }
    }
    return colliders;
}


///This function will parse the collider file and return the area of the room
/// This area will be the smallest rectangle that can contain all colliders
/// This function is NOT scheduled by bevy
fn get_area(backdrop_path: &String, room_location: &Transform) -> Rect {
    let mut collider_path: String = backdrop_path.replace("back", "cldr");
    collider_path = collider_path.replace(".png", ".svg");
    
    if IS_IN_WINDOWS{
        collider_path = collider_path.replace("textures", "Assets\\textures");
    }else{
        collider_path = collider_path.replace("textures", "Assets/textures");
    }

    warn!("Calculating room area using: {}", collider_path);

    let mut i = 0;

    let mut height = 0.0;
    let mut width = 0.0;
    if let Ok(lines) = read_lines(collider_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            warn!("{}", line);

            i += 1;
            if i > 2 {
                break;
            }
            //the rest logic is not useful for the first 2 lines so we will skip them after this.
            if i <= 2 {
                //line 2 contains the SVG info for width and height which we will use to find the size of the room
                if i == 2 {
                    let lineparts = line.split("\"").collect::<Vec<_>>();
                    width = lineparts[3].parse::<u32>().unwrap() as f32;
                    height = lineparts[5].parse::<u32>().unwrap() as f32;
                }
                continue;
            }
        }
    }
    
    let area = Rect::new(
        room_location.translation.x as f64, 
        room_location.translation.y as f64, 
        room_location.translation.x as f64 + (width * PIXEL_SCALE) as f64,
        room_location.translation.y as f64 + (height * PIXEL_SCALE) as f64
    );
    println!("Creating Room Area : {:?}", area.clone());
    area

    
}
