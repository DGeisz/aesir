use crate::neuron::SynapseType;

pub struct Reflex {
    sensor_name: String,
    actuator_name: String,
    synapse_type: SynapseType,
    weight: f32
}

impl Reflex {
    pub fn new(
        sensor_name: String,
        actuator_name: String,
        synapse_type: SynapseType,
        weight: f32
    ) -> Reflex {
        Reflex {
            sensor_name,
            actuator_name,
            synapse_type,
            weight
        }
    }
}