use std::f32::consts::PI;

use crate::{resources::MazeConfig, AppState};

use super::GameComponent;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};
use maze_generator::prelude::{Coordinates, Direction as MazeDirection, Maze};

const PLAN_SIZE: f32 = 5.;
const BORDER_HEIGHT: f32 = 0.3;
const ANGLE_INCREMENT: f32 = PI / 720.;

#[derive(Component)]
pub(crate) struct Floor;

#[derive(Component)]
pub(crate) struct Wall;

fn spwan_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor: Entity,
    length: f32,
    position: Vec3,
    rotation: Quat,
) {
    let mesh = Mesh::from(shape::Box::new(length, BORDER_HEIGHT, 0.001));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let wall = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
            transform: Transform::from_translation(position).with_rotation(rotation),
            ..default()
        })
        .insert(GameComponent)
        .insert(Wall)
        .insert(RigidBody::Fixed)
        .insert(collider)
        .id();
    commands.entity(floor).add_child(wall);
}

pub(crate) fn setup_board(
    mut commands: Commands,
    maze: Res<MazeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Perhaps, instead of handling walls by myself, use rapier joints (FixedJoint),
    // puting an anchor of both lower corner of the wall.

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 7., 4.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(GameComponent);

    // light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4., 8., 4.),
            ..default()
        })
        .insert(GameComponent);

    // Floor
    let mesh = Mesh::from(shape::Plane::from_size(PLAN_SIZE));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let floor = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            ..default()
        })
        .insert(GameComponent)
        .insert(Floor)
        .insert(RigidBody::KinematicPositionBased)
        .insert(collider)
        .id();

    // Front wall
    spwan_wall(
        &mut commands,
        &mut meshes,
        &mut materials,
        floor,
        PLAN_SIZE,
        Vec3::new(0., BORDER_HEIGHT / 2., PLAN_SIZE / 2.),
        Quat::IDENTITY,
    );

    // Back wall
    spwan_wall(
        &mut commands,
        &mut meshes,
        &mut materials,
        floor,
        PLAN_SIZE,
        Vec3::new(0., BORDER_HEIGHT / 2., -PLAN_SIZE / 2.),
        Quat::IDENTITY,
    );

    let rotation = Quat::from_rotation_y(PI / 2.);
    // Left wall
    spwan_wall(
        &mut commands,
        &mut meshes,
        &mut materials,
        floor,
        PLAN_SIZE,
        Vec3::new(-PLAN_SIZE / 2., BORDER_HEIGHT / 2., 0.),
        rotation,
    );

    // Right wall
    spwan_wall(
        &mut commands,
        &mut meshes,
        &mut materials,
        floor,
        PLAN_SIZE,
        Vec3::new(PLAN_SIZE / 2., BORDER_HEIGHT / 2., 0.),
        rotation,
    );

    // Spawn maze walls
    let maze: Maze = maze.as_ref().try_into().unwrap();
    let (width, height) = maze.size;
    let z_length = PLAN_SIZE / height as f32;
    let x_length = PLAN_SIZE / width as f32;
    for line in 0..height {
        for column in 0..width {
            // Optimization : we can spawn only on corner for room, here only north and west walls (missing walls
            // will spawn when handling right and south rooms).
            // Further optimization :
            // * spawn contiguous walls as one
            if let Some(field) = maze.get_field(&Coordinates::new(column, line)) {
                let x_position = (column as f32 * x_length) - (PLAN_SIZE / 2.);
                let z_position = (line as f32 * z_length) - (PLAN_SIZE / 2.);

                match field.field_type {
                    maze_generator::prelude::FieldType::Start => {
                        // Spawn marble
                        let mesh = Mesh::try_from(shape::Icosphere {
                            radius: 0.1,
                            subdivisions: 5,
                        })
                        .unwrap();
                        commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(mesh),
                                material: materials.add(Color::rgb(0., 0., 1.).into()),
                                ..default()
                            })
                            .insert(GameComponent)
                            .insert(RigidBody::Dynamic)
                            .insert(Collider::ball(0.1))
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(Restitution::coefficient(0.7))
                            .insert(TransformBundle::from(Transform::from_xyz(
                                x_position + x_length / 2.,
                                0.11,
                                z_position + z_length / 2.,
                            )))
                            .insert(GameComponent);
                    }
                    maze_generator::prelude::FieldType::Goal => {
                        // Spaw box with transparency and no collider to show the goal.
                        let mesh = Mesh::from(shape::Box::new(
                            x_length - 0.01,
                            BORDER_HEIGHT,
                            z_length - 0.01,
                        ));
                        let goal = commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(mesh),
                                material: materials.add(Color::rgba(0., 1., 0., 0.7).into()),
                                transform: Transform::from_translation(Vec3::new(
                                    x_position + x_length / 2.,
                                    BORDER_HEIGHT / 2.,
                                    z_position + z_length / 2.,
                                )),
                                ..Default::default()
                            })
                            .insert(GameComponent)
                            .id();
                        commands.entity(floor).add_child(goal);

                        // Spawn sensor to detect end of game
                        let goal_detection = commands
                            .spawn(Collider::segment(
                                Vec3::ZERO,
                                Vec3::new(0., BORDER_HEIGHT, 0.),
                            ))
                            .insert(Sensor)
                            .insert(TransformBundle::from_transform(
                                Transform::from_translation(Vec3::new(
                                    x_position + x_length / 2.,
                                    BORDER_HEIGHT / 2.,
                                    z_position + z_length / 2.,
                                ))
                                .with_rotation(Quat::from_rotation_y(-PI / 4.)),
                            ))
                            .insert(GameComponent)
                            .id();
                        commands.entity(floor).add_child(goal_detection);
                    }
                    maze_generator::prelude::FieldType::Normal => (),
                }

                if line != 0 && !field.has_passage(&MazeDirection::North) {
                    spwan_wall(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        floor,
                        x_length,
                        Vec3::new(x_position + x_length / 2., BORDER_HEIGHT / 2., z_position),
                        Quat::IDENTITY,
                    );
                }
                if column != 0 && !field.has_passage(&MazeDirection::West) {
                    spwan_wall(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        floor,
                        x_length,
                        Vec3::new(x_position, BORDER_HEIGHT / 2., z_position + z_length / 2.),
                        Quat::from_rotation_y(PI / 2.),
                    );
                }
            }
        }
    }

    // Add simple collider on top to prevent the marble from jumping out of the board
    let top = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(PLAN_SIZE / 2., 0., PLAN_SIZE / 2.))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0., BORDER_HEIGHT, 0.)),
        ))
        .insert(GameComponent)
        .id();
    commands.entity(floor).add_child(top);
}

pub(crate) fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mut floor: Query<&mut Transform, (With<Floor>, Without<Wall>)>,
) {
    if !keyboard.pressed(KeyCode::S)
        && !keyboard.pressed(KeyCode::Down)
        && !keyboard.pressed(KeyCode::Q)
        && !keyboard.pressed(KeyCode::Left)
        && !keyboard.pressed(KeyCode::Z)
        && !keyboard.pressed(KeyCode::Up)
        && !keyboard.pressed(KeyCode::D)
        && !keyboard.pressed(KeyCode::Right)
    {
        return;
    }
    if let Ok(mut floor) = floor.get_single_mut() {
        let angle_x = if keyboard.pressed(KeyCode::Z) || keyboard.pressed(KeyCode::Up) {
            -ANGLE_INCREMENT
        } else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            ANGLE_INCREMENT
        } else {
            0.
        };
        let angle_z = if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            -ANGLE_INCREMENT
        } else if keyboard.pressed(KeyCode::Q) || keyboard.pressed(KeyCode::Left) {
            ANGLE_INCREMENT
        } else {
            0.
        };
        floor.rotate_local_x(angle_x);
        floor.rotate_local_z(angle_z);
    }
}

pub(crate) fn detect_end_game(
    mut collision_events: EventReader<CollisionEvent>,
    mut state: ResMut<NextState<AppState>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(_, _, flags) = collision_event {
            if flags.intersects(CollisionEventFlags::SENSOR) {
                info!("Reach goal");
                state.set(AppState::Menu);
            }
        }
    }
}
