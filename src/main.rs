use astar::AStarPoint;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct PathFloor;

#[derive(Component)]
pub struct PathGoal;

#[derive(Component)]
pub struct PathStart;

#[derive(Component)]
pub struct EntityPather;

mod astar;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_startup_system(setup)
        .add_system(add_navmesh)
        .run();
}

fn add_navmesh(
    mut query: Query<&mut Transform, (With<PathFloor>, Without<EntityPather>)>,
    mut pather_query: Query<(&mut Velocity, &mut Transform), With<EntityPather>>,
    mut goal_query: Query<
        &mut Transform,
        (With<PathGoal>, Without<EntityPather>, Without<PathFloor>),
    >,
    mut start_query: Query<
        &mut Transform,
        (With<PathStart>, Without<EntityPather>, Without<PathFloor>, Without<PathGoal>),
    >,
) {
    let mut path_finder = astar::AStarPathFinder::default();

    for transform in &mut query {
        path_finder.add_point(AStarPoint {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
        });
    }

    let mut goal_location: Option<Vec3> = None;

    for transform in &mut goal_query {
        goal_location = Some(transform.translation);
    }

    let mut start_location: Option<Vec3> = None;

    for transform in &mut start_query {
        start_location = Some(transform.translation);
    }

    path_finder.add_connection(0, 1);
    path_finder.add_connection(1, 3);
    path_finder.add_connection(0, 2);

    for mut pather in &mut pather_query {
        let current_position = pather.1.translation;

        /*
        x: current_position.x,
        y: current_position.y,
        z: current_position.z, 
        */

        let path = path_finder.solve_path(
            AStarPoint {
                x: start_location.unwrap().x,
                y: start_location.unwrap().y,
                z: start_location.unwrap().z, 
            },
            AStarPoint {
                x: goal_location.unwrap().x,
                y: goal_location.unwrap().y,
                z: goal_location.unwrap().z,
            },
        );

        println!("GOAL: {:?}", goal_location.unwrap());
        if path.len() >= 1 {
            let point = &path[path.len() - 1];
            println!("point: {:?}", point);
            println!("location: {:?}", current_position);
            let delta_x = point.x - current_position.x;
            let delta_y = point.y - current_position.y;
            let delta_z = point.z - current_position.z;

            println!("{} {} {}", delta_x, delta_y, delta_z);
            pather.0.linvel = Vec3::new(delta_x, 0.0, delta_z);
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.1, 0.5))
        .insert(PathFloor);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            transform: Transform {
                translation: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.1, 0.5))
        .insert(PathFloor);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.1, 0.5))
        .insert(PathFloor);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            transform: Transform {
                translation: Vec3 {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.1, 0.5))
        .insert(PathFloor);

    /*commands
    .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
        transform: Transform { translation: Vec3{x: 1.0, y: 0.0, z: 1.0}, ..Default::default() },
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Collider::cuboid(0.5, 0.1, 0.5))
    .insert(PathFloor);*/

    // goal cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(5.0, 0.5, 2.5),
            ..default()
        })
        .insert(PathGoal)
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);

    
        commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(-2.5, 0.5, 2.5),
            ..default()
        })
        .insert(PathStart)
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.25f32,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(5.0, 0.5, 0.5),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.25))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity {..Default::default()})
        .insert(GravityScale {0: 0.0} )
        .insert(EntityPather);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
}
