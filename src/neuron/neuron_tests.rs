use crate::neuron::{ChargeCycle, FireTracker, Impulse, InternalCharge, SensoryNeuron, PlasticNeuron, Neuronic, TxNeuronic, ActuatorNeuron, SynapseType};

/// The following two tests are for Internal Charge
#[test]
fn test_incr_next_charge() {
    let bins = 8;
    let mut internal_charge = InternalCharge::new(bins);

    let weight = 15.;
    let measure = 0.2;

    internal_charge.incr_next_charge(ChargeCycle::Even, Impulse::new(weight, measure));

    assert_eq!(*internal_charge.odd_weights.get(&1).unwrap(), 15.);
    assert_eq!(*internal_charge.odd.get(&1).unwrap(), 3.);

    let weight = 14.;
    let measure = 0.15;

    internal_charge.incr_next_charge(ChargeCycle::Even, Impulse::new(weight, measure));

    assert_eq!(*internal_charge.odd_weights.get(&1).unwrap(), 29.);
    assert_eq!((*internal_charge.odd.get(&1).unwrap() * 10.) as i32, 51);

    let weight = 15.;
    let measure = 0.2;

    internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

    assert_eq!(*internal_charge.even_weights.get(&1).unwrap(), 15.);
    assert_eq!(*internal_charge.even.get(&1).unwrap(), 3.);

    let weight = 14.;
    let measure = 0.15;

    internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

    assert_eq!(*internal_charge.even_weights.get(&1).unwrap(), 29.);
    assert_eq!((*internal_charge.even.get(&1).unwrap() * 10.) as i32, 51);
}

#[test]
fn test_get_and_reset_charge() {
    let bins = 8;
    let mut internal_charge = InternalCharge::new(bins);
    let weight = 15.;
    let measure = 0.2;
    internal_charge.incr_next_charge(ChargeCycle::Even, Impulse::new(weight, measure));
    let weight = 14.;
    let measure = 0.15;
    internal_charge.incr_next_charge(ChargeCycle::Even, Impulse::new(weight, measure));

    assert_eq!(
        (internal_charge.get_charge_weighted_average(ChargeCycle::Odd) * 1000.).round() as i32,
        176
    );

    compare_f32(internal_charge.get_charge_weighted_average(ChargeCycle::Odd), 0.175);

    compare_f32(internal_charge.get_weights(ChargeCycle::Odd), 29.);


    let weight = 15.;
    let measure = 0.2;
    internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));
    let weight = 14.;
    let measure = 0.15;
    internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

    let weight = 10.;
    let measure = 0.5;
    internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

    compare_f32(internal_charge.get_charge_weighted_average(ChargeCycle::Even), 0.258);

    compare_f32(internal_charge.get_weights(ChargeCycle::Even), 39.);

    internal_charge.reset_charge(ChargeCycle::Odd);

    assert_eq!(*internal_charge.odd_weights.get(&1).unwrap(), 0.);
    assert_eq!(*internal_charge.odd.get(&1).unwrap(), 0.);

    internal_charge.reset_charge(ChargeCycle::Even);

    assert_eq!(*internal_charge.even_weights.get(&1).unwrap(), 0.);
    assert_eq!(*internal_charge.even.get(&1).unwrap(), 0.);
}

/// This tests FireTracker
#[test]
fn test_fire_tracker() {
    let mut tracker = FireTracker::new();
    tracker.create_receipt(ChargeCycle::Even, true, 0.3);

    let even_receipt = tracker.check_receipt(ChargeCycle::Even);
    let odd_receipt = tracker.check_receipt(ChargeCycle::Odd);

    assert_eq!(even_receipt.measure, 0.3);
    assert_eq!(even_receipt.fired, true);
    assert_eq!(odd_receipt.measure, 0.0);
    assert_eq!(odd_receipt.fired, false);

    tracker.create_receipt(ChargeCycle::Odd, true, 0.6);

    let even_receipt = tracker.check_receipt(ChargeCycle::Even);
    let odd_receipt = tracker.check_receipt(ChargeCycle::Odd);

    assert_eq!(even_receipt.measure, 0.3);
    assert_eq!(even_receipt.fired, true);
    assert_eq!(odd_receipt.measure, 0.6);
    assert_eq!(odd_receipt.fired, true);
}

fn weight_modifier(target_measure: f32, weight_measure: f32) -> f32 {
    let x = (target_measure - weight_measure).abs();
    let denominator = 1. - (-1.5_f32).exp();

    let numerator = (-15. * x).exp() - (-1.5_f32).exp();

    return numerator / denominator;
}

/// Compares two floats to three decimal places
fn compare_f32(float1: f32, float2: f32) {
    assert_eq!((float1 * 1000.).floor(), (float2 * 1000.).floor());
}

/// Neuron tests
#[test]
fn test_sensor_plastic_fire() {
    let bins = 8;

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 6.;
    let sensor2_synapse_weight = 5.;

    sensor1.add_plastic_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_plastic_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.618);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);
}

#[test]
fn test_sensor_plastic_fire_with_inhibition() {
    let bins = 8;

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);
    let s3 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;
    let s3_measure = 0.45;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);
    s3.set_measure(s3_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 9.;
    let sensor2_synapse_weight = 5.;
    let s3_weight = 3.;

    sensor1.add_plastic_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_plastic_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);
    s3.add_plastic_synapse(s3_weight, SynapseType::Inhibitory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.713);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);


    //Do it all again, but this time the inhibitory synapse blocks everything
    //I'm not going to write the equivalent of this test for plastic synapses,
    //just please for the love of God, don't fuck up.  Just implement fire_synapses
    //for inhibitory neurons in the same way

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);
    let s3 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;
    let s3_measure = 0.45;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);
    s3.set_measure(s3_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 11.;
    let sensor2_synapse_weight = 5.;
    let s3_weight = 7.;

    sensor1.add_plastic_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_plastic_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);
    s3.add_plastic_synapse(s3_weight, SynapseType::Inhibitory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.8);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);
}

#[test]
fn test_sensor_static_fire() {
    let bins = 8;

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 6.;
    let sensor2_synapse_weight = 5.;

    sensor1.add_static_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_static_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.618);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);
}

#[test]
fn test_sensor_static_fire_with_inhibition() {
    let bins = 8;

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);
    let s3 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;
    let s3_measure = 0.45;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);
    s3.set_measure(s3_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 9.;
    let sensor2_synapse_weight = 5.;
    let s3_weight = 3.;

    sensor1.add_static_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_static_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);
    s3.add_static_synapse(s3_weight, SynapseType::Inhibitory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.713);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);

    //Do it all again, but this time the inhibitory synapse blocks everything
    //I'm not going to write the equivalent of this test for plastic synapses,
    //just please for the love of God, don't fuck up.  Just implement fire_synapses
    //for inhibitory neurons in the same way

    // Create sensors
    let sensor1 = SensoryNeuron::new(weight_modifier);
    let sensor2 = SensoryNeuron::new(weight_modifier);
    let s3 = SensoryNeuron::new(weight_modifier);

    // Set sensors
    let sensor1_measure = 0.8;
    let sensor2_measure = 0.4;
    let s3_measure = 0.45;

    sensor1.set_measure(sensor1_measure);
    sensor2.set_measure(sensor2_measure);
    s3.set_measure(s3_measure);

    // Create plastic
    let fire_threshold = 10.;

    let plastic = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    // Make synapses
    let sensor1_synapse_weight = 11.;
    let sensor2_synapse_weight = 5.;
    let s3_weight = 7.;

    sensor1.add_static_synapse(sensor1_synapse_weight, SynapseType::Excitatory, &plastic);
    sensor2.add_static_synapse(sensor2_synapse_weight, SynapseType::Excitatory, &plastic);
    s3.add_static_synapse(s3_weight, SynapseType::Inhibitory, &plastic);

    // Run cycles
    let cycle = ChargeCycle::Even;

    sensor1.run_cycle(cycle);
    sensor2.run_cycle(cycle);
    plastic.run_cycle(cycle);

    // Check odd internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Odd), 0.8);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Odd), 11.);

    // Check even internal charge
    compare_f32(plastic.internal_charge.borrow().get_charge_weighted_average(ChargeCycle::Even), 0.0);
    compare_f32(plastic.internal_charge.borrow().get_weights(ChargeCycle::Even), 0.0);
}

#[test]
fn test_plastic_plastic_fire_to_actuator() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    // Set sensors
    s1.set_measure(0.3);
    s2.set_measure(0.4);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p2 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);
    neurons.push(&p2);

    // Make sp synapses
    s1.add_plastic_synapse(7., SynapseType::Excitatory, &p1);
    s1.add_plastic_synapse(5., SynapseType::Excitatory, &p2);

    s2.add_plastic_synapse(6., SynapseType::Excitatory, &p1);
    s2.add_plastic_synapse(6.5, SynapseType::Excitatory, &p2);

    // Create actuator
    let act = ActuatorNeuron::new(bins, fire_threshold);

    neurons.push(&act);

    // Make pa synapses
    p1.add_plastic_synapse(5.5, SynapseType::Excitatory, &act);
    p2.add_plastic_synapse(6., SynapseType::Excitatory, &act);

    // Run three cycles
    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    compare_f32(act.read_measure(), 0.351);
}

#[test]
fn test_plastic_plastic_fire_to_actuator_with_inhibition() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    // Set sensors
    s1.set_measure(0.3);
    s2.set_measure(0.4);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p2 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p3 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);
    neurons.push(&p2);

    // Make sp synapses
    s1.add_plastic_synapse(7., SynapseType::Excitatory, &p1);
    s1.add_plastic_synapse(5., SynapseType::Excitatory, &p2);
    s1.add_plastic_synapse(8., SynapseType::Excitatory, &p3);

    s2.add_plastic_synapse(6., SynapseType::Excitatory, &p1);
    s2.add_plastic_synapse(6.5, SynapseType::Excitatory, &p2);
    s2.add_plastic_synapse(3., SynapseType::Excitatory, &p3);

    // Create actuator
    let act = ActuatorNeuron::new(bins, fire_threshold);

    neurons.push(&act);

    // Make pa synapses
    p1.add_plastic_synapse(5.5, SynapseType::Excitatory, &act);
    p2.add_plastic_synapse(8., SynapseType::Excitatory, &act);
    p3.add_plastic_synapse(2., SynapseType::Inhibitory, &act);

    // Run three cycles
    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    compare_f32(act.read_measure(), 0.356);
}

#[test]
fn test_plastic_static_fire_to_actuator() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    // Set sensors
    s1.set_measure(0.3);
    s2.set_measure(0.4);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p2 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);
    neurons.push(&p2);

    // Make sp synapses
    s1.add_static_synapse(7., SynapseType::Excitatory, &p1);
    s1.add_static_synapse(5., SynapseType::Excitatory, &p2);

    s2.add_static_synapse(6., SynapseType::Excitatory, &p1);
    s2.add_static_synapse(6.5, SynapseType::Excitatory, &p2);

    // Create actuator
    let act = ActuatorNeuron::new(bins, fire_threshold);

    neurons.push(&act);

    // Make pa synapses
    p1.add_static_synapse(5.5, SynapseType::Excitatory, &act);
    p2.add_static_synapse(6., SynapseType::Excitatory, &act);

    // Run three cycles
    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    compare_f32(act.read_measure(), 0.351);
}

#[test]
fn test_plastic_static_fire_to_actuator_with_inhibition() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    // Set sensors
    s1.set_measure(0.3);
    s2.set_measure(0.4);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p2 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p3 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);
    neurons.push(&p2);

    // Make sp synapses
    s1.add_static_synapse(7., SynapseType::Excitatory, &p1);
    s1.add_static_synapse(5., SynapseType::Excitatory, &p2);
    s1.add_static_synapse(8., SynapseType::Excitatory, &p3);

    s2.add_static_synapse(6., SynapseType::Excitatory, &p1);
    s2.add_static_synapse(6.5, SynapseType::Excitatory, &p2);
    s2.add_static_synapse(3., SynapseType::Excitatory, &p3);

    // Create actuator
    let act = ActuatorNeuron::new(bins, fire_threshold);

    neurons.push(&act);

    // Make pa synapses
    p1.add_static_synapse(5.5, SynapseType::Excitatory, &act);
    p2.add_static_synapse(8., SynapseType::Excitatory, &act);
    p3.add_static_synapse(2., SynapseType::Inhibitory, &act);

    // Run three cycles
    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    compare_f32(act.read_measure(), 0.356);
}

#[test]
fn test_basic_weight_change() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    let s1_measure = 0.5;
    let s2_measure = 0.7;

    // Set sensors
    s1.set_measure(s1_measure);
    s2.set_measure(s2_measure);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);

    let s1_weight = 7.2;
    let s2_weight = 5.8;

    // Make sp synapses
    s1.add_plastic_synapse(s1_weight, SynapseType::Excitatory, &p1);
    s2.add_plastic_synapse(s2_weight, SynapseType::Excitatory, &p1);

    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let p_charge = ((s1_weight * s1_measure) + (s2_weight * s2_measure)) / (s1_weight + s2_weight);
    let new_s1_weight = s1_weight + weight_modifier(p_charge, s1_measure);
    let new_s2_weight = s2_weight + weight_modifier(p_charge, s2_measure);

    let actual_s1_weight = s1.plastic_synapses.borrow().get(0).unwrap().weight;
    let actual_s2_weight = s2.plastic_synapses.borrow().get(0).unwrap().weight;

    compare_f32(new_s1_weight, actual_s1_weight);
    compare_f32(new_s2_weight, actual_s2_weight);
}

#[test]
fn test_plastic_weight_change() {
    let bins = 8;
    let fire_threshold = 10.;

    let mut neurons = Vec::<&dyn Neuronic>::new();

    // Create sensors
    let s1 = SensoryNeuron::new(weight_modifier);
    let s2 = SensoryNeuron::new(weight_modifier);

    neurons.push(&s1);
    neurons.push(&s2);

    let s1_measure = 0.3;
    let s2_measure = 0.4;

    // Set sensors
    s1.set_measure(s1_measure);
    s2.set_measure(s2_measure);

    // Create plastic
    let p1 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);
    let p2 = PlasticNeuron::new(bins, weight_modifier, fire_threshold);

    neurons.push(&p1);
    neurons.push(&p2);

    let s1_p1_weight = 7.;
    let s1_p2_weight = 5.;

    let s2_p1_weight = 6.;
    let s2_p2_weight = 6.5;


    // Make sp synapses
    s1.add_plastic_synapse(s1_p1_weight, SynapseType::Excitatory, &p1);
    s1.add_plastic_synapse(s1_p2_weight, SynapseType::Excitatory, &p2);

    s2.add_plastic_synapse(s2_p1_weight, SynapseType::Excitatory, &p1);
    s2.add_plastic_synapse(s2_p2_weight, SynapseType::Excitatory, &p2);

    // Create actuator
    let act = ActuatorNeuron::new(bins, fire_threshold);

    neurons.push(&act);

    let p1_act_weight = 5.5;
    let p2_act_weight = 6.;

    // Make pa synapses
    p1.add_plastic_synapse(p1_act_weight, SynapseType::Excitatory, &act);
    p2.add_plastic_synapse(p2_act_weight, SynapseType::Excitatory, &act);

    // Run three cycles
    let cycle = ChargeCycle::Even;
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let cycle = cycle.next_cycle();
    for neuron in &neurons {
        neuron.run_cycle(cycle);
    }

    let p1_charge = ((s1_measure * s1_p1_weight) + (s2_measure * s2_p1_weight)) / (s1_p1_weight + s2_p1_weight);
    let p2_charge = ((s1_measure * s1_p2_weight) + (s2_measure * s2_p2_weight)) / (s1_p2_weight + s2_p2_weight);

    let act_charge = ((p1_act_weight * p1_charge) + (p2_act_weight * p2_charge)) / (p1_act_weight + p2_act_weight);

    let p1_new_weight = p1_act_weight + weight_modifier(act_charge, p1_charge);
    let p2_new_weight = p2_act_weight + weight_modifier(act_charge, p2_charge);

    let p1_actual_weight = p1.plastic_synapses.borrow().get(0).unwrap().weight;
    let p2_actual_weight = p2.plastic_synapses.borrow().get(0).unwrap().weight;

    compare_f32(p1_new_weight, p1_actual_weight);
    compare_f32(p2_new_weight, p2_actual_weight);
}
