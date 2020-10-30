use std::collections::HashMap;

pub trait EcpGeometry {
    fn new(num_plastic: u32, num_actuator: u32, num_sensory: u32, nearby_count: u32) -> Self
    where
        Self: Sized;

    fn get_num_plastic(&self) -> u32;
    fn get_num_sensory(&self) -> u32;
    fn get_num_actuator(&self) -> u32;

    fn get_nearby_count(&self) -> u32;

    fn first_plastic_loc(&self) -> Vec<i32>;
    fn next_plastic_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;
    fn first_actuator_loc(&self) -> Vec<i32>;
    fn next_actuator_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;
    fn first_sensory_loc(&self) -> Vec<i32>;
    fn next_sensory_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>>;

    /// First index is plastic neurons, second index is
    /// actuator neurons,
    fn get_nearby_rx_neurons(&self, loc: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>);
}

pub struct EcpBox {
    num_plastic: u32,
    num_actuator: u32,
    num_sensory: u32,
    nearby_count: u32,
    plastic_side_length: u32,
    sensory_side_length: u32,
    nearby_side_length: u32,
    plastic_to_actuator_connections: HashMap<Vec<i32>, Vec<Vec<i32>>>
}

impl EcpGeometry for EcpBox {
    fn new(num_plastic: u32, num_actuator: u32, num_sensory: u32, nearby_count: u32) -> Self where
        Self: Sized {

        let plastic_side_length = (num_plastic as f32).powf(1. / 3.).floor() as u32;

        if plastic_side_length.pow(3) != num_plastic {
            panic!("num_plastic must be a perfect cube");
        }

        let nearby_side_length = ((nearby_count + 1) as f32).powf(1. / 3.).floor() as u32;

        if nearby_side_length.pow(3) != nearby_count + 1 {
            panic!("nearby_count must be one less than a perfect cube");
        }

        let mut sensory_side_length = (num_sensory as f32).powf(0.5).floor() as u32;

        if sensory_side_length.pow(2) != num_sensory {
            sensory_side_length += 1;
        }

        let mut plastic_to_actuator_connections = HashMap::<Vec<i32>, Vec<Vec<i32>>>::new();

        let mut act_x = 0;
        let act_y = plastic_side_length as i32;
        let mut act_z = 0;

        //Line up actuator neurons across the bottom of the opposing y plane
        loop {
            if (act_z * plastic_side_length as i32) + act_x > num_actuator as i32 {
                break;
            }

            let half_nearby = (nearby_side_length as f32 / 2 as f32).floor() as i32;
            let nearby_index = (nearby_side_length - 1) as i32;

            let mut x_0 = act_x - half_nearby;
            let y_0 = (plastic_side_length - nearby_side_length) as i32;
            let mut z_0 = act_z - half_nearby;

            if x_0 < 0 {
                x_0 = 0;
            } else if plastic_side_length as i32 - x_0 < nearby_side_length as i32{
                x_0 = (plastic_side_length - nearby_side_length) as i32;
            }

            if z_0 < 0 {
                z_0 = 0;
            } else if plastic_side_length as i32 - z_0 < nearby_side_length as i32{
                z_0 = (plastic_side_length - nearby_side_length) as i32;
            }

            let mut x = 0;
            let mut y = 0;
            let mut z = 0;

            loop {
                if x == nearby_index && y == nearby_index && z == nearby_index {
                    break;
                }

                match plastic_to_actuator_connections.get_mut(&vec![x_0 + x, y_0 + y, z_0 + z]) {
                    Some(vec) => {
                        vec.push(vec![act_x, act_y, act_z]);
                    },
                    None => {
                        plastic_to_actuator_connections.insert(vec![x, y, z], vec![vec![act_x, act_y, act_z]]);
                    }
                }

                if x == nearby_index {
                    if y == nearby_index {
                        x = 0;
                        y = 0;
                        z += 1;
                    } else {
                        x = 0;
                        y += 1;
                    }
                } else {
                    x += 1;
                }
            }

            if act_x + 1 == plastic_side_length as i32 {
                act_x = 0;
                act_z += 1;
            } else {
                act_x += 1;
            }
        }

        EcpBox {
            num_plastic,
            num_sensory,
            num_actuator,
            nearby_count,
            plastic_side_length,
            sensory_side_length,
            nearby_side_length,

            plastic_to_actuator_connections
        }
    }

    fn get_num_plastic(&self) -> u32 {
        self.num_plastic
    }

    fn get_num_sensory(&self) -> u32 {
        self.num_sensory
    }

    fn get_num_actuator(&self) -> u32 {
        self.num_actuator
    }

    fn get_nearby_count(&self) -> u32 {
        self.nearby_count
    }

    fn first_plastic_loc(&self) -> Vec<i32> {
        vec![0, 0, 0]
    }

    fn next_plastic_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>> {
        let x = *loc.get(0).unwrap();
        let y = *loc.get(1).unwrap();
        let z = *loc.get(2).unwrap();

        let plastic_index = self.plastic_side_length as i32 - 1;

        if x == plastic_index && y == plastic_index && z == plastic_index {
            return None;
        }

        return if x == plastic_index {
            if y == plastic_index {
                Some(vec![0, 0, z + 1])
            } else {
                Some(vec![0, y + 1, z])
            }
        } else {
            Some(vec![x + 1, y, z])
        };
    }

    fn first_actuator_loc(&self) -> Vec<i32> {
        vec![0, self.plastic_side_length as i32, 0]
    }

    fn next_actuator_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>> {
        let x = *loc.get(0).unwrap();
        let y = *loc.get(1).unwrap();
        let z = *loc.get(2).unwrap();

        let plastic_index = self.plastic_side_length as i32 - 1;

        if z * self.plastic_side_length as i32 + x + 1 == self.num_actuator as i32 {
            return None;
        }

        return if x == plastic_index {
            Some(vec![0, y, z + 1])
        } else {
            Some(vec![x + 1, y, z])
        };
    }

    fn first_sensory_loc(&self) -> Vec<i32> {
        vec![0, -1, 0]
    }

    fn next_sensory_loc(&self, loc: &Vec<i32>) -> Option<Vec<i32>> {
        let x = *loc.get(0).unwrap();
        let y = *loc.get(1).unwrap();
        let z = *loc.get(2).unwrap();

        let sensor_index = self.sensory_side_length as i32 - 1;

        if x == sensor_index && z == sensor_index {
            return None;
        }

        return if x == sensor_index {
            Some(vec![0, y, z + 1])
        } else {
            Some(vec![x + 1, y, z])
        }
    }

    fn get_nearby_rx_neurons(&self, loc: &Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let loc_x = *loc.get(0).unwrap();
        let loc_y = *loc.get(1).unwrap();
        let loc_z = *loc.get(2).unwrap();

        let nearby_index = (self.nearby_side_length - 1) as i32;

        if loc_y < 0 {
            //This is a sensor
            let scaled_plastic = self.plastic_side_length - self.nearby_side_length;

            let x_0 = ((scaled_plastic as f32 / self.sensory_side_length as f32) * loc_x as f32).floor() as i32;
            let y_0 = 0;
            let z_0 = ((scaled_plastic as f32 / self.sensory_side_length as f32) * loc_z as f32).floor() as i32;

            let mut x = 0;
            let mut y = 0;
            let mut z = 0;

            let mut plastic_locs = Vec::new();

            loop {
                plastic_locs.push(vec![x_0 + x, y_0 + y, z_0 + z]);

                if x == nearby_index {
                    if y == nearby_index {
                        x = 0;
                        y = 0;
                        z += 1;
                    } else {
                        x = 0;
                        y += 1;
                    }
                } else {
                    x += 1;
                }

                if x == nearby_index && y == nearby_index && z == nearby_index {
                    return (plastic_locs, Vec::new());
                }
            }
        } else {
            let half_nearby = (self.nearby_side_length as f32 / 2.0).floor() as i32;
            let actuator_connections = self.plastic_to_actuator_connections.get(loc);

            let actuators = match actuator_connections {
                Some(vec) => vec.clone(),
                None => Vec::new(),
            };

            let actuator_count = actuators.len();

            let mut x_0 = loc_x - half_nearby;
            let mut y_0 = loc_y - half_nearby;
            let mut z_0 = loc_z - half_nearby;

            if x_0 < 0 {
                x_0 = 0;
            } else if self.plastic_side_length as i32 - x_0 < self.nearby_side_length as i32 {
                x_0 = (self.plastic_side_length - self.nearby_side_length) as i32;
            }

            if y_0 < 0 {
                y_0 = 0;
            } else if self.plastic_side_length as i32 - y_0 < self.nearby_side_length as i32 {
                y_0 = (self.plastic_side_length - self.nearby_side_length) as i32;
            }

            if z_0 < 0 {
                z_0 = 0;
            } else if self.plastic_side_length as i32 - z_0 < self.nearby_side_length as i32 {
                z_0 = (self.plastic_side_length - self.nearby_side_length) as i32;
            }

            let mut x = 0;
            let mut y = 0;
            let mut z = 0;

            let mut count = 1;

            let mut plastic_connections = Vec::new();

            loop {
                if count > actuator_count {
                    plastic_connections.push(vec![x + x_0, y + y_0, z + z_0]);
                }

                if x == nearby_index {
                    if y == nearby_index {
                        x = 0;
                        y = 0;
                        z += 1;
                    } else {
                        x = 0;
                        y += 1;
                    }
                } else {
                    x += 1;
                }

                count += 1;

                if x == nearby_index && y == nearby_index && z == nearby_index {
                    return (plastic_connections, actuators);
                }
            }
        }
    }
}

#[cfg(test)]
pub mod ecp_tests;
