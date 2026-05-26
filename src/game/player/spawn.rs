use avian3d::prelude::*;
use bevy::prelude::*;

use super::components as player;

pub fn spawn_test_player(mut cmds: Commands) {
    let player_body = player::Body::default();

    // FUTURE: keep spawning simulation only, model scenes,
    // camera ownership and visuals should be in the client
    // presentation files so they can change without changing
    // gameplay
    cmds.spawn((
        Name::new("TestPlayer"),
        player::Player,
        player::LocalPlayer,
        player::Controller,
        player_body,
        player::Input::default(),
        player::LookState::default(),
        player::MoveState::default(),
        player::GroundProbe::default(),
        Collider::capsule(player_body.radius, player_body.height),
        Transform::from_xyz(0.0, 2.0, 0.0),
        TransformInterpolation,
    ))
    .with_children(|parent| {
        parent
            .spawn((
                Name::new("PlayerVisualRoot"),
                player::VisRoot,
                Transform::IDENTITY,
            ))
            .with_children(|visual_root| {
                visual_root.spawn((
                    Name::new("PlayerCameraAnchor"),
                    player::CamAnchor,
                    player::LCamAnchor,
                    Transform::from_xyz(0.0, player_body.eye_height, 0.0),
                ));
            });
    });
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
