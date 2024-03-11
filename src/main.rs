use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(ClearColor(Color::default().with_l(0.1)))
        .run();
}

#[derive(Component)]
struct Ball;

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn(Camera2dBundle::default());
    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_scale(Vec2::splat(20.).extend(1.)),
            ..default()
        },
        Ball,
    ));
}
