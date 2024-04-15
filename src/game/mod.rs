use bevy::a11y::accesskit::Rect;
use bevy::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use super::resources::*;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum InGameState {
    #[default]
    Loading,
    Running,
    //Cutscene,
    //RoomChange
}


pub fn game_plugin(app: &mut App) {
    app.init_state::<InGameState>()

    .add_systems(OnEnter(InGameState::Loading), load_room)

    .add_systems(OnEnter(GameState::Running), create_game_objects)
    .add_systems(Update,player_movement.run_if(in_state(InGameState::Running)));
}

//Component Used to tag the player
#[derive(Component)]
struct Player;

//Component Used to tag a Static Object that does nothing
#[derive(Component)]
struct StaticObject;

fn create_game_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let room_texture = asset_server.load("Textures/Rooms/Room-concept-01.png");
    commands.spawn(
        (SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new((96.0 * 11.0), (96.0 * 8.0))),
                .. default()
            },
            texture: room_texture,
            .. default()
        },
        StaticObject
    )
    );
    
    let player_texture = asset_server.load("Textures/Player/Player-Singlet.png");

    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(96.0, 96.0)),
                flip_x: false,
                .. default()
            },
            texture: player_texture,
            .. default()
        },
        Player
    ));

    
}

fn player_movement(
    mut players: Query<(&mut Transform, &Player, &mut Sprite)>,
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
    }
}

#[derive(Component)]
enum ColliderType{
    RIGID,
    INTERACTABLE,
    TRIGGER
}

#[derive(Component)]
struct Collider {
    rect: Rect,
    style: ColliderType,
}

//Helper function for loading files
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn load_room(
    commands: Commands,
    server: Res<AssetServer>,
    current_room: Res<CurrentRoom>
){
    let level = current_room.0;
    let room = current_room.1;
    let var = current_room.2;


    let mut file_name = String::new();
    
    file_name = format!("assets\\Maps\\Room-col{}{}{}.svg", level, room, var).to_string();
    println!("{}", file_name);

    let mut i =0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            //seems like for each line of this file we can create a new collider object
            //lets parse out the important info first
            i += 1;
            if (i<=2) | ((i-2)%6 <= 0){
                continue;
            }
            let x:i32;
            let y:i32;
            let w:i32;
            let h:i32;

            let pretty_line = line.trim();
            let parts = pretty_line.split("\" ").collect::<Vec<_>>();

            for part in parts{
                //println!("{part}");
                let comp = part.split("\"").collect::<Vec<_>>();
                match comp.get(0){
                    _ => println!("{:?}", comp.get(1)),
                }
                

            //     if ch.is_digit(10) {
            //         intBuilder =  format!("{:?}", intBuilder + &ch.to_string());
            //     }
            //     else if intBuilder.is_empty(){
            //         intBuilder  = "".to_string();
            //     }else{
            //         println!("Built INT: {}", intBuilder);
            //         intBuilder  = "".to_string();
            //     }
            }


        }
    }


}