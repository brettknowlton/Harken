use bevy::{app::AppExit, prelude::*};
use super::{despawn_screen, resources::*};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MainMenuState {
    #[default]
    Splash,
    Main,
    SettingMain,
    SettingsDisplay,
    SettingsSound,
    Disabled,
}

pub fn main_menu_plugin(app: &mut App) {
    app 
        .init_state::<MainMenuState>()
        //.add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
        
        .add_systems(OnEnter(MainMenuState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(MainMenuState::Splash)))
        .add_systems(OnExit(MainMenuState::Splash), despawn_screen::<OnSplashScreen>)

        .add_systems(OnEnter(MainMenuState::Main), alt_menu_setup)
        .add_systems(OnExit(MainMenuState::Main), despawn_screen::<OnMainMenu>)

        //settings menu systems:
        .add_systems(
            OnEnter(MainMenuState::SettingMain), 
            settings_menu_setup
        )
        .add_systems(
            OnExit(MainMenuState::SettingMain),
            despawn_screen::<OnSettingsMainMenu>,
        )

        //display
        .add_systems(OnEnter(MainMenuState::SettingsDisplay), display_settings_menu_setup)
        .add_systems(
            Update, 
            (setting_button::<DisplayQuality>.run_if(in_state(MainMenuState::SettingsDisplay)), )
        )
        .add_systems(
            OnExit(MainMenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenu>,
        )

        //sound
        .add_systems(OnEnter(MainMenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            (setting_button::<Volume>.run_if(in_state(MainMenuState::SettingsSound)), 
        ))
        .add_systems(
            OnExit(MainMenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenu>,
        )

        //systems common to all settings menus
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::MainMenu))
        );

    }

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenu;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMainMenu;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenu;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenu;

//colors of buttons in different states
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const CRIMSON : Color = Color::srgb(0.863, 0.078, 0.235);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Continue,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}
//Dispaly and formatting for SettingsMainMenu
fn settings_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}
//Display and formatting for SettingsSound Menu
fn sound_settings_menu_setup (mut commands: Commands, volume: Res<Volume>){
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnSoundSettingsMenu,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: CRIMSON.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Volume",
                                    button_text_style.clone(),
                                ));
                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                    let mut entity = parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(30.0),
                                                height: Val::Px(65.0),
                                                ..button_style.clone()
                                            },
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        Volume(volume_setting),
                                    ));
                                    if *volume == Volume(volume_setting) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToSettings,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back", button_text_style));
                            });
                    });
            });
}
//Display and formatting for DisplaySetting Menu
fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnDisplaySettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn(TextBundle::from_section(
                                "Display Quality",
                                button_text_style.clone(),
                            ));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    quality_setting,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        format!("{quality_setting:?}"),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}


/// Display and formatting for MainMenu, this is an old version and may just need to be deleted...

// fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>){
//     println!("Entered Main Menu");
//
//     // Common style for all buttons on the screen
//     let button_style = Style {
//         width: Val::Px(250.0),
//         height: Val::Px(65.0),
//         margin: UiRect::all(Val::Px(20.0)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..default()
//     };
//     let button_icon_style = Style {
//         width: Val::Px(30.0),
//         // This takes the icons out of the flexbox flow, to be positioned exactly
//         position_type: PositionType::Absolute,
//         // The icon will be close to the left border of the button
//         left: Val::Px(10.0),
//         ..default()
//     };
//     let button_text_style = TextStyle {
//         font_size: 40.0,
//         color: TEXT_COLOR,
//         ..default()
//     };
//
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Percent(100.0),
//                     height: Val::Percent(100.0),
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             OnMainMenu,
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn(
//                     NodeBundle {
//                         style: Style {
//                             flex_direction: FlexDirection::Column,
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         background_color: CRIMSON.into(),
//                         ..default()
//                 })
//                 .with_children(|parent| {
//                     // Display the game name
//                     parent.spawn(
//                         TextBundle::from_section(
//                             "harken",
//                             TextStyle {
//                                 font_size: 80.0,
//                                 color: TEXT_COLOR,
//                                 ..default()
//                             },
//                         )
//                         .with_style(Style {
//                             margin: UiRect::all(Val::Px(50.0)),
//                             ..default()
//                         }),
//                     );
//
//                     // Display three buttons for each action available from the main menu:
//                     // - new game
//                     // - settings
//                     // - quit
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Play,
//                         ))
//                         .with_children(|parent| {
//                             let icon = asset_server.load("textures/Game Icons/right.png");
//                             parent.spawn(ImageBundle {
//                                 style: button_icon_style.clone(),
//                                 image: UiImage::new(icon),
//                                 ..default()
//                             });
//                             parent.spawn(TextBundle::from_section(
//                                 "New Game",
//                                 button_text_style.clone(),
//                             ));
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Continue,
//                         ))
//                         .with_children(|parent|{
//                             let icon = asset_server.load("textures/Game Icons/right.png");
//                             parent.spawn(ImageBundle {
//                                 style: button_icon_style.clone(),
//                                 image: UiImage::new(icon),
//                                 ..default()
//                             });
//                             parent.spawn(TextBundle::from_section(
//                                 "Continue",
//                                 button_text_style.clone(),
//                             ));
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style.clone(),
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Settings,
//                         ))
//                         .with_children(|parent| {
//                             let icon = asset_server.load("textures/Game Icons/wrench.png");
//                             parent.spawn(ImageBundle {
//                                 style: button_icon_style.clone(),
//                                 image: UiImage::new(icon),
//                                 ..default()
//                             });
//                             parent.spawn(TextBundle::from_section(
//                                 "Settings",
//                                 button_text_style.clone(),
//                             ));
//                         });
//                     parent
//                         .spawn((
//                             ButtonBundle {
//                                 style: button_style,
//                                 background_color: NORMAL_BUTTON.into(),
//                                 ..default()
//                             },
//                             MenuButtonAction::Quit,
//                         ))
//                         .with_children(|parent| {
//                             let icon = asset_server.load("textures/Game Icons/exitRight.png");
//                             parent.spawn(ImageBundle {
//                                 style: button_icon_style,
//                                 image: UiImage::new(icon),
//                                 ..default()
//                             });
//                             parent.spawn(TextBundle::from_section("Quit", button_text_style));
//                         });
//                 });
//         }); // 
//     }

fn alt_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let background_texture = asset_server.load("textures/Roomsbackground.png");

    commands.spawn((
        SpriteBundle{
            sprite: Sprite{
                custom_size: Some(Vec2::new(1056.0, 768.0)),
                .. default()
            },
            texture: background_texture,
            .. default()
        }, 
        OnMainMenu
    ));


        // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        //left: Val::Percent(33.3),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };


    commands.spawn((
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            .. default()
        },
    .. default()
    },
    OnMainMenu
))
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            })
        .with_children(|parent|{
            parent.spawn(
                TextBundle::from_section(
                    "harken",
                    TextStyle {
                        font_size: 80.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            //add main menu buttons
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Play,
            ))

            .with_children(|parent| {
                let icon = asset_server.load("textures/Game Icons/right.png");
                parent.spawn(ImageBundle {
                    style: button_icon_style.clone(),
                    image: UiImage::new(icon),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "New Game",
                    button_text_style.clone(),
                ));
            });
            
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Continue,
            ))
            .with_children(|parent|{
                let icon = asset_server.load("textures/Game Icons/right.png");
                parent.spawn(ImageBundle {
                    style: button_icon_style.clone(),
                    image: UiImage::new(icon),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "Continue",
                    button_text_style.clone(),
                ));
            });
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Settings,
            ))
            .with_children(|parent| {
                let icon = asset_server.load("textures/Game Icons/wrench.png");
                parent.spawn(ImageBundle {
                    style: button_icon_style.clone(),
                    image: UiImage::new(icon),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    button_text_style.clone(),
                ));
            });
            parent.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Quit,
            ))
            .with_children(|parent| {
                let icon = asset_server.load("textures/Game Icons/exitRight.png");
                parent.spawn(ImageBundle {
                    style: button_icon_style,
                    image: UiImage::new(icon),
                    ..default()
                });
                parent.spawn(TextBundle::from_section("Quit", button_text_style));
            });
        });
    });
}
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>
){
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(GameState::LevelLoading);
                    menu_state.set(MainMenuState::Disabled);
                    current_level.0 = 1;
                }
                MenuButtonAction::Continue => {
                    game_state.set(GameState::Loading);
                    menu_state.set(MainMenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MainMenuState::SettingMain);
                }
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MainMenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MainMenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MainMenuState::Main);
                }
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MainMenuState::SettingMain);
                }
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
){
    for(interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
#[derive(Component)]
struct OnSplashScreen;

//todo: understand these derives
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
){
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("Textures/ham&bread.png");
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
                    width: Val::Px(800.0),
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
    mut menu_state: ResMut<NextState<MainMenuState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
){
    //println!("Ticking Countdown");
    if timer.tick(time.delta()).finished() {
        menu_state.set(MainMenuState::Main);
    }
}