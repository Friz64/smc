pub mod camera;
pub mod gameplay;
pub mod mainmenu;

// used as a resource for systems
#[derive(PartialEq)]
pub enum CurrentState {
    Loading,
    MainMenu,
    Gameplay,
}
