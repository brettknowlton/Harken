use bevy::{
    prelude::*,
    diagnostic::FrameTimeDiagnosticsPlugin,
    core::FrameCount,
};


mod resources;
mod menu;
mod game;

const SCREEN_WIDTH: f32 = 1056.0;
const SCREEN_HEIGHT: f32 = 768.0;

const PIXEL_SCALE: f32 = SCREEN_HEIGHT / 8.0;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Harken".into(),
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        resizable: false,
                        decorations: true,
                        visible: false,
                        ..default()
                    }),
                    ..default()

            })
            .build(),
        )

        .insert_resource(resources::DebugMode(true))

        .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 0.0)))
        .insert_resource(resources::DisplayQuality::Medium,)
        .insert_resource(resources::Volume(7))
        .insert_resource(resources::CurrentRoom(1, 0, 0))

        .add_systems(Startup, setup)

        .init_state::<resources::GameState>()

        .add_plugins(FrameTimeDiagnosticsPlugin,)
        .add_systems(Update, make_visible)


        .add_plugins(menu::main_menu_plugin)

        .add_plugins(game::game_plugin)
        .run();

    println!("Goodbye!");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    mut commands: Commands
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}