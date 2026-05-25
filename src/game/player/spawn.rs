use avian3d::prelude::*;
use bevy::prelude::*;

use super::components::{
    GroundProbe, Player, PlayerBody, PlayerController, PlayerInput, PlayerMovementState,
};

pub fn spawn_test_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_body = PlayerBody::default();

    // TODO: replace the default capsule with a
    // structure that can handle custom rigged models
    cmds.spawn((
        Name::new("TestPlayer"),
        Player,
        PlayerController,
        player_body,
        PlayerInput::default(),
        PlayerMovementState::default(),
        GroundProbe::default(),
        Collider::capsule(player_body.radius, player_body.height),
        Mesh3d(meshes.add(Capsule3d::new(player_body.radius, player_body.height))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        TransformInterpolation,
    ));
}

pub fn spawn_test_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: replace this map later on
    cmds.spawn((
        Name::new("TestFloor"),
        RigidBody::Static,
        Collider::cuboid(40.0, 1.0, 40.0),
        Mesh3d(meshes.add(Cuboid::new(40.0, 1.0, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    cmds.spawn((
        Name::new("TestWall"),
        RigidBody::Static,
        Collider::cuboid(2.0, 3.0, 8.0),
        Mesh3d(meshes.add(Cuboid::new(2.0, 3.0, 8.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(4.0, 1.5, 0.0),
    ));
}
