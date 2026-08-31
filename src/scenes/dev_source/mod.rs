use avian3d::prelude::*;
use bevy::prelude::*;

pub fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Test floor. (Static RigidBody)
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 1.0, 50.0),
        Mesh3d(meshes.add(Cuboid::new(50.0, 1.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    // Ramp for "trimping" / surfing tests. (Static RigidBody)
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 1.0, 20.0),
        Mesh3d(meshes.add(Cuboid::new(10.0, 1.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.8))),
        Transform::from_xyz(10.0, 2.0, 0.0).with_rotation(Quat::from_rotation_z(0.5)),
    ));

    // Sun. (Directional Light)
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 10.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
