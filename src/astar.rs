use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AStarPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AStarPoint {
    pub fn calculate_distance(&self, other: &AStarPoint) -> f32 {
        let delta_x = (self.x - other.x).abs();
        let delta_y = (self.y - other.y).abs();
        let delta_z = (self.z - other.z).abs();

        let distance = (delta_x * delta_x + delta_y * delta_y + delta_z * delta_z).sqrt();

        distance
    }
}

#[derive(Debug, Clone, Copy)]
struct AStarConnection {
    pub node_a_id: usize,
    pub node_b_id: usize,
}

#[derive(Debug, Clone, Default)]
pub struct AStarPathFinder {
    points: Vec<AStarPoint>,
    connections: Vec<AStarConnection>,
}

impl AStarPathFinder {
    pub fn add_point(&mut self, point: AStarPoint) -> bool {
        self.points.push(point);
        true
    }

    pub fn add_connection(&mut self, node_a_id: usize, node_b_id: usize) -> bool {
        if self.points.get(node_a_id).is_none() || self.points.get(node_b_id).is_none() {
            return false;
        }

        self.connections.push(AStarConnection {
            node_a_id,
            node_b_id,
        });
        false
    }

    fn get_connections(&self, node_id: usize) -> Vec<usize> {
        let mut connections: Vec<usize> = Vec::new();

        for connection in &self.connections {
            if connection.node_a_id == node_id {
                connections.push(connection.node_b_id);
            }

            if connection.node_b_id == node_id {
                connections.push(connection.node_a_id);
            }
        }

        connections
    }

    pub fn get_closest_point(&self, current_point: AStarPoint) -> Option<(usize, AStarPoint)> {
        let mut closest_point: Option<(usize, AStarPoint)> = None;
        let mut closest_distance: f32 = f32::MAX;

        for point in self.points.iter().enumerate() {
            let distance = current_point.calculate_distance(point.1);

            if distance < closest_distance {
                closest_point = Some((point.0.clone(), point.1.clone()));
                closest_distance = distance;
            }
        }

        if closest_point.is_some() {
            return Some(closest_point.unwrap());
        } else {
            return None;
        }
    }

    pub fn solve_path(&mut self, start: AStarPoint, goal: AStarPoint) -> Option<Vec<AStarPoint>> {
        let start_point = match self.get_closest_point(start) {
            Some(x) => x,
            None => {
                return None;
            }
        };

        let goal_point = match self.get_closest_point(goal) {
            Some(x) => x,
            None => {
                return None;
            }
        };

        let mut open_set: Vec<usize> = vec![start_point.0];
        let mut processed: Vec<usize> = Vec::new();

        let mut g_costs: HashMap<usize, f32> = HashMap::new();
        let mut h_costs: HashMap<usize, f32> = HashMap::new();
        let mut path_connections: HashMap<usize, usize> = HashMap::new();

        for (node_id, point) in self.points.iter().enumerate() {
            g_costs.insert(node_id, point.calculate_distance(&start_point.1));
            h_costs.insert(node_id, point.calculate_distance(&goal_point.1));
        }

        while !open_set.is_empty() {
            let mut current_id = open_set[0];
            let mut current_point = self.points[current_id];

            for point_id in &open_set {
                let point = self.points[*point_id];

                let point_g = point.calculate_distance(&start_point.1);
                let point_h = point.calculate_distance(&goal_point.1);
                let point_f = point_g + point_h;

                g_costs.insert(*point_id, point_g);
                h_costs.insert(*point_id, point_h);

                let current_g = current_point.calculate_distance(&start_point.1);
                let current_h = current_point.calculate_distance(&goal_point.1);
                let current_f = current_g + current_h;

                if point_f <= current_f && point_h < current_h {
                    current_id = *point_id;
                    current_point = self.points[current_id];
                }
            }

            processed.push(current_id);
            open_set.remove(0);

            let mut connections = self.get_connections(current_id);
            connections.retain(|id| if processed.contains(id) { false } else { true });

            for conn_id in connections {
                let in_search = open_set.contains(&conn_id);

                let cost_to_neighbour =
                    g_costs[&conn_id] + current_point.calculate_distance(&self.points[conn_id]);

                if !in_search || cost_to_neighbour < g_costs[&conn_id] {
                    g_costs.insert(conn_id, cost_to_neighbour);
                    path_connections.insert(conn_id, current_id);

                    if !in_search {
                        h_costs.insert(
                            conn_id,
                            self.points[conn_id].calculate_distance(&goal_point.1),
                        );
                        open_set.push(conn_id);
                    }
                }
            }
        }

        let mut route: Vec<AStarPoint> = Vec::new();

        let mut point_id = goal_point.0;

        loop {
            route.push(self.points[point_id]);

            match path_connections.get(&point_id) {
                Some(x) => point_id = *x,
                None => {
                    break;
                }
            };
        }

        if route[0] != goal_point.1 || route[route.len() - 1] != start_point.1 {
            return None;
        }

        route.reverse();

        return Some(route);
    }
}
