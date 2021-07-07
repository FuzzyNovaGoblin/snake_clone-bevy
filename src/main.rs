use bevy::{
    prelude::{App, Commands, Handle, IntoSystem, OrthographicCameraBundle},
    sprite::ColorMaterial,
    DefaultPlugins,
};


struct SnakeHead;
struct Materials {
    head_materials: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
