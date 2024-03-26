use bevy::prelude::*;
mod menu;

mod resources;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .insert_resource(resources::DisplayQuality::Medium)
        .insert_resource(resources::Volume(7))

        .add_systems(Startup, setup)

        .init_state::<resources::GameState>()

        .add_plugins(menu::main_menu_plugin)
        .run();

    println!("Hello, world!");
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