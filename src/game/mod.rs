use bevy::a11y::accesskit::Rect;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use std::fs::{DirEntry, File};
use std::io::{self, BufRead};
use std::path::Path;

use log::{debug, warn};

use crate::{is_in_windows, PIXEL_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::resources::*;

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::LevelLoading), (load_level_room_data, create_game_objects))

        .add_systems(Update, (
            spawn_colliders,
            display_rooms,
        ).chain().run_if(in_state(GameState::Loading)))

        .add_systems(Update, player_movement.run_if(in_state(GameState::Running)))
        .add_systems(Update, collision_detection.run_if(in_state(GameState::Running)))
        .add_systems(Update, move_camera.run_if(in_state(GameState::Running)))

        .add_systems(Update, room_status.run_if(in_state(GameState::Running)));
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
    let mut player_texture: Handle<Image> =
        asset_server.load("Textures\\Player\\Player-Singlet.png");

    if !is_in_windows() {
        player_texture = asset_server.load("Textures/Player/Player-Singlet.png");
    }
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.625, 2.0)),
                flip_x: false,
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: player_texture,
            transform: Transform {
                scale: Vec3 {
                    x: PIXEL_SCALE,
                    y: PIXEL_SCALE,
                    z: 0.0,
                },
                ..Default::default()
            },
            ..default()
        },
        Player {
            vel_x: 0.0,
            vel_y: 0.0,

            hitbox: Rect::new(0.0, 0.0, PIXEL_SCALE as f64, PIXEL_SCALE as f64 * 0.625),
        },
    ));
}

fn display_rooms(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    rooms: Query<&Room>,

    mut game_state: ResMut<NextState<GameState>>,
) {
    for room in &rooms {

        let mut p_rect: Rect;
        //check if the player's hitbox intersects with the room's area
        
        if room.active {
            //this is an active room that should be displayed
            println!("Attempting to display room: {:?}", room.backdrop_path);
            let backdrop = asset_server.load(room.backdrop_path.clone());
            commands.spawn(SpriteBundle {
                sprite: Sprite { ..default() },
                texture: backdrop,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    scale: Vec3::new(6.0, 6.0, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }

    game_state.set(GameState::Running);
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
    ChangeLevel,
    ChangeVar,
    ChangeState,
}

#[derive(Component, Clone, Copy)]
struct Collider {
    transform: Transform,
    style: ColliderType,
}

#[derive(Component)]
struct Room {
    level: u32,
    room: u32,
    var: u32,

    active: bool,

    area: Rect,

    backdrop_path: String,
    colliders: Vec<Collider>,
}

fn room_status(
    mut rooms: Query<&mut Room>,
    players: Query<(&Transform, &Player)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for mut room in &mut rooms {
        //set room to active it the room's rect intersects with the player's rect
        for (player_transform, _) in &players {
            let px_scale: f64 = PIXEL_SCALE as f64;
            let p_left: f64 = player_transform.translation.x as f64;
            let p_right: f64 = p_left + px_scale;

            let p_bot: f64 = player_transform.translation.y as f64;
            let py_scale: f64 = player_transform.scale.y as f64;
            let p_top: f64 = p_bot + py_scale;

            let p_rect = Rect::new(p_left, p_bot, p_right, p_top);

            if room.area.intersect(p_rect).area() != 0.0 {
                if room.active {
                    println!("Player is intersecting room: {}{}{}", room.level, room.room, room.var);
                    //player is in this room and it is already active
                    return;
                } else {
                    //player is intersecting an inactive room, we now need to re-load rooms and display
                    room.active = true;
                    game_state.set(GameState::Loading);
                }
            }else{
                //player is not intersecting this room
                if room.active {
                    //we are intersecting an inactive room that should be de-activated
                    room.active= false;
                }
            }
        }
    }
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

fn load_level_room_data(
    mut commands: Commands,
    in_debug: Res<DebugMode>,
    asset_server: Res<AssetServer>,
    current_room: Res<CurrentRoom>,

    mut game_state: ResMut<NextState<GameState>>,
) {
    //for each file in the rooms folder load the room data
    let rooms_path: String;
    if is_in_windows() {
        rooms_path = format!("Assets\\Textures\\Rooms\\L{}", current_room.0);
    } else {
        rooms_path = format!("Assets\\Textures/Rooms/L{}", current_room.0);
    }
    println!("Looking for rooms in: {}", rooms_path);

    let paths = std::fs::read_dir(rooms_path);
    match paths {
        Ok(_) => {
            info!("Found Rooms Folder");
        }
        Err(_) => {
            warn!("Could not read rooms folder");
        }
    }

    let mut bg_paths = Vec::<String>::new();
    let mut cl_paths = Vec::<String>::new();
    let mut fg_paths = Vec::<String>::new();

    for item in paths.unwrap() {
        match item {
            Ok(item) => {
                let mut item_name: String = item.path().display().to_string();
                if is_in_windows() {
                    item_name = item_name.replace("Assets\\", "");
                } else {
                    item_name = item_name.replace("Assets/", "");
                }
                info!("Found file: {}", item_name.clone());

                //remove the assets folder from the path as the asset server will add it back
                
                

                if item_name.contains("back") {
                    bg_paths.push(item_name);
                } else if item_name.contains("cldr") {
                    cl_paths.push(item_name);
                } else if item_name.contains("fore") {
                    fg_paths.push(item_name);
                }
            }
            Err(_) => {
                warn!("Could not read item in rooms folder");
            }
        }
    }


        for item in bg_paths {//why the fuck is this

            let colliders = load_colliders(
                item.clone(),
                in_debug.0
            );

            println!("Spawning Room with backdrop: {:?}", item);
            commands.spawn(Room {
                level: current_room.0,
                room: current_room.1,
                var: current_room.2,

                active: true,
                area: get_area(&item),
                backdrop_path: item.clone(),
                colliders: colliders.clone(),
            }
        );
    }

    game_state.set(GameState::Loading);
}


fn spawn_colliders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    rooms: Query<&Room>,
    in_debug: Res<DebugMode>,
) {


    for room in &rooms {
        if(room.active){
            for collider in &room.colliders {
                commands.spawn(*collider);
    
                if in_debug.0 {
                    info!("A collider was created at: {:?}", collider.transform);
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            anchor: Anchor::TopLeft,
                            ..default()
                        },
                        transform: collider.transform,
                        texture: asset_server.load("Textures\\Rooms\\cldr.png"),
                        ..default()
                    });
                }
            }
        }
    }
}


///This function will parse the collider file and return the area of the room
/// This area will be the smallest rectangle that can contain all colliders
/// This function is NOT scheduled by bevy
fn get_area(backdrop_path: &String) -> Rect {
    let mut collider_path: String = backdrop_path.replace("back", "cldr");
    collider_path = collider_path.replace(".png", ".svg");
    
    if is_in_windows(){
        collider_path = collider_path.replace("Textures", "Assets\\textures");
    }else{
        collider_path = collider_path.replace("Textures", "Assets/textures");
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
    println!("Creating Room Area with coordinates: 0,0,{},{}", width * PIXEL_SCALE, height * PIXEL_SCALE);

    Rect::new(0.0, 0.0, (width * PIXEL_SCALE) as f64,(height * PIXEL_SCALE) as f64)
}


///This function will parse the collider file and return a vector of colliders
/// This function is NOT scheduled by bevy
fn load_colliders(backdrop_path: String, in_debug: bool,) -> Vec<Collider> {

    let mut collider_path: String = backdrop_path.replace("back", "cldr");
    collider_path = collider_path.replace(".png", ".svg");
    
    if is_in_windows(){
        collider_path = collider_path.replace("Textures", "Assets\\textures");
    }else{
        collider_path = collider_path.replace("Textures", "Assets/textures");
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
                println!("End of file: {}", line);
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
                        println!("Parsed: {}", part.parse::<i16>().unwrap());
                        ints.push(part.parse::<i16>().unwrap());
                    }

                    Err(_) => {
                        if part.contains("#") {
                            println!("Found Color: {}", part);
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
            println!("COLOR FOUND: {}", col);

            // println!("Creating Collider with x:{} y:{} w:{} h:{} of type:{:?}", ((x as f32*PIXEL_SCALE)  - SCREEN_WIDTH/2.0), ((SCREEN_HEIGHT / 2.0) + (y as f32*PIXEL_SCALE) as f32), w as f32*PIXEL_SCALE, h as f32*PIXEL_SCALE, st);
            colliders.push(Collider {
                // transform: Rect::new((x*96).into(), (y*96).into(), ((x+w)*96).into(), ((y+h)*96).into()),
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 * PIXEL_SCALE) + PIXEL_SCALE * 2.0 - (SCREEN_WIDTH / 2.0),
                        (-3.5 * PIXEL_SCALE + (SCREEN_HEIGHT / 2.0)) - (y as f32 * PIXEL_SCALE),
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
