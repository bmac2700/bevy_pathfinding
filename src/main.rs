use astar::AStarPoint;
use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

#[derive(Component)]
pub struct PathFloor;

#[derive(Component)]
pub struct PathGoal;

#[derive(Component)]
pub struct PathStart;

#[derive(Component)]
pub struct VisualisedPath;

mod astar;
mod world_generation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .insert_resource(astar::AStarPathFinder::default())
        .add_startup_system(setup)
        .add_startup_system(crate::world_generation::generate_world)
        .add_system(show_path)
        //.add_system(add_navmesh)
        .run();
}

fn show_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut path_finder: ResMut<crate::astar::AStarPathFinder>,
    mut goal_query: Query<&mut Transform, (With<PathGoal>, Without<PathFloor>)>,
    mut start_query: Query<
        &mut Transform,
        (With<PathStart>, Without<PathFloor>, Without<PathGoal>),
    >,
    mut visualised_path_query: Query<
        Entity,
        (
            With<VisualisedPath>,
            Without<PathFloor>,
            Without<PathGoal>,
            Without<PathStart>,
        ),
    >,
) {
    for e in &mut visualised_path_query {
        commands.entity(e).despawn();
    }

    let mut goal_location: Option<Vec3> = None;

    for transform in &mut goal_query {
        goal_location = Some(transform.translation);
    }

    let mut start_location: Option<Vec3> = None;

    for transform in &mut start_query {
        start_location = Some(transform.translation);
    }

    let path = match path_finder.solve_path(
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
    ) {
        Some(v) => v,
        None => {
            //println!("No path");
            return;
        }
    };

    if path.len() >= 1 {
        for p in path {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.25f32,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    transform: Transform::from_xyz(p.x, p.y, p.z),
                    ..default()
                })
                .insert(VisualisedPath);
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
}
