use bevy::{
    prelude::*,
    diagnostic::FrameTimeDiagnosticsPlugin,
    core::FrameCount,
};


mod resources;
mod menu;
mod game;
mod dev_tools;

const SCREEN_WIDTH: f32 = 1056.0;
const SCREEN_HEIGHT: f32 = 768.0;

const PIXEL_SCALE: f32 = SCREEN_HEIGHT / 16.0;

const IS_IN_WINDOWS: bool = cfg!(target_os = "windows");

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


        //RESOURCES BABY!
        .insert_resource(resources::DebugMode(true))
        .insert_resource(resources::DevMode(true))

        .add_plugins(dev_tools::dev_tools)

        .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))


        .insert_resource(resources::DisplayQuality::Medium,)
        .insert_resource(resources::Volume(7))
        .insert_resource(resources::CurrentLevel(1))


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