use bevy::prelude::*;
use rand::Rng;

use crate::{astar::AStarPoint, PathFloor};

pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut path_finder: ResMut<crate::astar::AStarPathFinder>,
) {
    /*path_finder.add_point(AStarPoint {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });*/

    const X: usize = 32;
    const Y: usize = 32;

    let mut grid: Vec<Vec<usize>> = Vec::new();

    for x in 0..X {
        let mut layer: Vec<usize> = Vec::new();

        for y in 0..Y {
            layer.push(x * X + y);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32,
                            y: 0.0,
                            z: y as f32,
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    ..default()
                })
                .insert(PathFloor);

            path_finder.add_point(AStarPoint {
                x: x as f32,
                y: 0.0,
                z: y as f32,
            });
        }

        grid.push(layer);
    }

    for (x, layer) in grid.iter().enumerate() {
        for (y, node_id) in layer.iter().enumerate() {
            if x + 1 < X && y < Y {
                path_finder.add_connection(*node_id, grid[x + 1][y]);
            }

            if x < X && y + 1 < Y {
                path_finder.add_connection(*node_id, grid[x][y + 1]);
            }

            let mut rng = rand::thread_rng();

            let num: u8 = rng.gen_range(0..=3);
            if num == 1 {
                path_finder.remove_connections(*node_id);
                commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32,
                            y: 0.1,
                            z: y as f32,
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    ..default()
                });
            }
        }
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 100000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(X as f32 / 2.0, 8.0, Y as f32 / 2.0),
        ..default()
    });
}
