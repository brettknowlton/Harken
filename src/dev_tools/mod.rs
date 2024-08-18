use bevy::prelude::*;

use crate::{IS_IN_WINDOWS, PIXEL_SCALE};

use super::game::{
    interaction::{Facing, Interactable},
    Player, Shadow,
};
use super::resources::{CurrentLevel, DebugMode, DevMode};

pub fn dev_tools(app: &mut App) {
    app.add_systems(
        Update,
        (
            insert_interactable, save_interactables
        ).run_if(resource_exists_and_equals(DevMode(true))),
    );
}

fn insert_interactable(
    mut commands: Commands,
    players: Query<(Entity, &Player, &Transform), Without<Shadow>>,
    input: Res<ButtonInput<KeyCode>>,
    in_debug: Res<DebugMode>,
    in_dev: Res<DevMode>,
    asset_server: Res<AssetServer>,
) {
    if !in_dev.0 {
        return;
    }

    let _ = players;
    //detect a keyboard press, when the "I" key is pressed, spawn an interactable object
    if input.just_pressed(KeyCode::KeyI) {
        let player = players.iter().next().unwrap();
        let player_transform = player.2;

        //snap all interactales to the nearest rounded grid space (PIXEL_SCALE)
        let x = (player_transform.translation.x / PIXEL_SCALE).round() * PIXEL_SCALE;
        let y = (player_transform.translation.y / PIXEL_SCALE).round() * PIXEL_SCALE;

        let interactable = Interactable::new(
            Rect::new(x, y, x + PIXEL_SCALE, y + PIXEL_SCALE),
            vec![Facing::Up, Facing::Down, Facing::Left, Facing::Right],
        );

        commands.spawn(interactable);

        if in_debug.0 {
            let tex;

            if IS_IN_WINDOWS {
                tex = asset_server.load("textures\\rooms\\cldr.png");
            } else {
                tex = asset_server.load("textures/rooms/cldr.png");
            }

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 50.0),
                    scale: Vec3::new(PIXEL_SCALE, PIXEL_SCALE, 1.0),
                    ..Default::default()
                },
                texture: tex,

                ..Default::default()
            });
        }
    }
}

fn save_interactables(
    interactables: Query<&Interactable>,
    current_room: Res<CurrentLevel>,

    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let mut data: Vec<serde_json::Value> = Vec::new();

        for interactable in &interactables {
            info!("Gathering Interactable for write: {:?}", interactable);
            data.push(serde_json::to_value(interactable).unwrap());
        }

        let json_data = serde_json::to_string(&data).unwrap();
        warn!("Saving Json payload:\n{}", json_data);

        let path = format!(
            "assets/textures/rooms/L{}/interactables.json",
            current_room.0
        );

        if IS_IN_WINDOWS {
            write_json(&json_data, &path.replace("/", "\\"))
                .expect("Issue Creating or Writing file");
        } else {
            write_json(&json_data, &path).expect("Issue Creating or Writing file");
        }

        info!("Interactables saved to: {}", path);
    }
}

fn write_json(json_string: &str, file_path: &str) -> Result<(), std::io::Error> {
    info!("Attempting to create file: {}", file_path);
    std::fs::write(file_path, json_string.as_bytes())?;
    info!("File Created Successfully");
    Ok(())
}


