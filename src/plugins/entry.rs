// TODO: seperate this into modular pieces
use bevy::prelude::*;

pub struct BasePlugin;

pub fn setup(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn(Camera2d::default());
    command.spawn(Text::new("penis"));
    command.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
