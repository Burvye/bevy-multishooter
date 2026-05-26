use avian3d::prelude::*;
use bevy::prelude::*;

use super::components as player;
use crate::client::PlayerCamera;

pub fn spawn_test_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_body = player::Body::default();

    // TODO: replace the default capsule with a
    // structure that can handle custom rigged models
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
                    Name::new("PlayerDebugBody"),
                    Mesh3d(meshes.add(Capsule3d::new(player_body.radius, player_body.height))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    Transform::IDENTITY,
                ));

                visual_root
                    .spawn((
                        Name::new("PlayerCameraAnchor"),
                        player::CamAnchor,
                        player::LCamAnchor,
                        Transform::from_xyz(0.0, player_body.eye_height, 0.0),
                    ))
                    .with_children(|camera_anchor| {
                        camera_anchor.spawn((
                            PlayerCamera,
                            Camera3d::default(),
                            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::NEG_Z, Vec3::Y),
                        ));
                    });
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
