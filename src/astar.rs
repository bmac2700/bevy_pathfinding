#[derive(Debug, Clone, Copy)]
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

    fn get_closest_point(&self, current_point: AStarPoint) -> Option<(usize, AStarPoint)> {
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

    pub fn solve_path(&mut self, start: AStarPoint, goal: AStarPoint) -> Vec<AStarPoint> {
        let start_point = match self.get_closest_point(start) {
            Some(x) => x,
            None => {
                return Vec::new();
            }
        };

        let goal_point = match self.get_closest_point(goal) {
            Some(x) => x,
            None => {
                return Vec::new();
            }
        };

        let open_set: Vec<usize> = vec![start_point.0];

        while !open_set.is_empty() {

            break;
        }

        return vec![start_point.1];
    }
}


//G = Distance from start
//H = Distance from end
//F = G + H