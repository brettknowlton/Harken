use bevy::prelude::*;

use super::{despawn_screen, resources::GameState, resources::DisplayQuality, resources::Volume};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum InGameState {
    #[default]
    Running,
    Cutscene,
    RoomChange
}


pub fn game_plugin(app: &mut App) {
    app.init_state::<InGameState>()
    .add_systems(OnEnter(GameState::Running), create_game_objects)
    .add_systems(Update,player_movement.run_if(in_state(InGameState::Running)));
}

//Component Used to ta
#[derive(Component)]
struct Player;


fn create_game_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let texture = asset_server.load("Textures/Player/Player-Singlet.png");

    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(96.0, 96.0)),
                flip_x: false,
                .. default()
            },
            texture,
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