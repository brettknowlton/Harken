use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Running,
    Dead,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);



#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct CurrentRoom(pub u32, pub u32, pub u32);
//1st: Level Number
//2nd: Room Id
//3rd: Room Varation
