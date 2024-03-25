use bevy::prelude::*;
use super::{despawn_screen, GameState};


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MainMenuState {
    #[default]
    Splash,
    Main,
    SettingMain,
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .init_state::<MainMenuState>()
        .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)

        .add_systems(OnEnter(MainMenuState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(MainMenuState::Splash)))
        .add_systems(OnExit(MainMenuState::Splash), despawn_screen::<OnSplashScreen>);

    }

fn main_menu_setup(

){
    println!("Entered Main Menu");
}

#[derive(Component)]
struct OnSplashScreen;

//todo: understand these derives
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);


fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("gangsta_kirby.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

//this counts down and progresses the lifetime of the splash screen (2 seconds?)
fn countdown(
    mut game_state: ResMut<NextState<MainMenuState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
){
    if timer.tick(time.delta()).finished() {
        game_state.set(MainMenuState::Main);
    }
}