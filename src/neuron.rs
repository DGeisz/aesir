use std::cell::RefCell;
use std::collections::HashMap;

/// For better documentation of everything, see the eywa library
/// Most of the names here are equivalent

/// Here, the impulse measure is always between 0 and 1
#[derive(Copy, Clone)]
pub struct Impulse {
    weight: f32,
    measure: f32,
}

impl Impulse {
    pub fn new(weight: f32, measure: f32) -> Impulse {
        Impulse { weight, measure }
    }

    pub fn weighted_measure(&self) -> f32 {
        self.weight * self.measure
    }
}

pub trait Neuronic {
    fn run_cycle(&self, cycle: ChargeCycle);
}

pub trait TxNeuronic {
    fn fire_synapses(&self, cycle: ChargeCycle, current_fire_receipt: &FireReceipt, prev_prev_fire_receipt: &FireReceipt);
    fn add_plastic_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic);
    fn add_static_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic);
}

pub trait RxNeuronic {
    fn intake_synaptic_impulse(&self, impulse: Impulse);

    fn get_fire_receipt(&self, cycle: ChargeCycle) -> FireReceipt;
}

#[derive(Copy, Clone)]
pub enum ChargeCycle {
    Even,
    Odd,
}

impl ChargeCycle {
    fn next_cycle(&self) -> ChargeCycle {
        match self {
            ChargeCycle::Even => ChargeCycle::Odd,
            ChargeCycle::Odd => ChargeCycle::Even,
        }
    }

    fn prev_cycle(&self) -> ChargeCycle {
        self.next_cycle()
    }
}

pub struct InternalCharge {
    even: HashMap<u8, f32>,
    even_weights: HashMap<u8, f32>,
    odd: HashMap<u8, f32>,
    odd_weights: HashMap<u8, f32>,
    bins: u8, //Number of bins
}

impl InternalCharge {
    fn new(bins: u8) -> InternalCharge {
        InternalCharge {
            even: (0..bins).map(|i| (i, 0.0)).collect::<HashMap<u8, f32>>(),
            even_weights: (0..bins).map(|i| (i, 0.0)).collect::<HashMap<u8, f32>>(),
            odd: (0..bins).map(|i| (i, 0.0)).collect::<HashMap<u8, f32>>(),
            odd_weights: (0..bins).map(|i| (i, 0.0)).collect::<HashMap<u8, f32>>(),
            bins,
        }
    }

    fn get_charge_weighted_average(&self, cycle: ChargeCycle) -> f32 {
        match cycle {
            ChargeCycle::Even => {
                let mut total_weighted_charge = 0.0;
                let mut weights = 0.0;

                for i in 0..self.bins {
                    let weight = *self.even_weights.get(&i).unwrap();
                    if weight >= 0.0 {
                        weights += weight;

                        let weighted_charge = *self.even.get(&i).unwrap();
                        if weighted_charge >= 0.0 {
                            total_weighted_charge += weighted_charge;
                        }
                    }
                }

                if weights == 0.0 {0.0} else {total_weighted_charge / weights}
            }
            ChargeCycle::Odd => {
                let mut total_weighted_charge = 0.0;
                let mut weights = 0.0;

                for i in 0..self.bins {
                    let weight = *self.odd_weights.get(&i).unwrap();
                    if weight >= 0.0 {
                        weights += weight;

                        let weighted_charge = *self.odd.get(&i).unwrap();
                        if weighted_charge >= 0.0 {
                            total_weighted_charge += weighted_charge;
                        }
                    }
                }

                if weights == 0.0 {0.0} else {total_weighted_charge / weights}
            }
        }
    }

    fn get_weights(&self, cycle: ChargeCycle) -> f32 {
        match cycle {
            ChargeCycle::Even => {
                self.even_weights.values().sum::<f32>()
            },
            ChargeCycle::Odd => {
                self.odd_weights.values().sum::<f32>()
            }
        }
    }

    fn reset_charge(&mut self, cycle: ChargeCycle) {
        match cycle {
            ChargeCycle::Even => {
                self.even = (0..self.bins)
                    .map(|i| (i, 0.0))
                    .collect::<HashMap<u8, f32>>();
                self.even_weights = (0..self.bins)
                    .map(|i| (i, 0.0))
                    .collect::<HashMap<u8, f32>>();
            }
            ChargeCycle::Odd => {
                self.odd = (0..self.bins)
                    .map(|i| (i, 0.0))
                    .collect::<HashMap<u8, f32>>();
                self.odd_weights = (0..self.bins)
                    .map(|i| (i, 0.0))
                    .collect::<HashMap<u8, f32>>();
            }
        }
    }

    fn incr_next_charge(&mut self, cycle: ChargeCycle, incr_charge: Impulse) {
        let bin = self.get_bin(incr_charge.measure);

        match cycle.next_cycle() {
            ChargeCycle::Even => {
                *self.even.get_mut(&bin).unwrap() += incr_charge.weighted_measure();
                *self.even_weights.get_mut(&bin).unwrap() += incr_charge.weight;
            }
            ChargeCycle::Odd => {
                *self.odd.get_mut(&bin).unwrap() += incr_charge.weighted_measure();
                *self.odd_weights.get_mut(&bin).unwrap() += incr_charge.weight;
            }
        }
    }

    /// Called when an inhibitory synapse fires
    fn inhibit_next_charge(&mut self, cycle: ChargeCycle, inhibitory_impulse: Impulse) {
        let bin = self.get_bin(inhibitory_impulse.measure);

        match cycle.next_cycle() {
            ChargeCycle::Even => {
                *self.even.get_mut(&bin).unwrap() -= inhibitory_impulse.weighted_measure();
                *self.even_weights.get_mut(&bin).unwrap() -= inhibitory_impulse.weight;
            }
            ChargeCycle::Odd => {
                *self.odd.get_mut(&bin).unwrap() -= inhibitory_impulse.weighted_measure();
                *self.odd_weights.get_mut(&bin).unwrap() -= inhibitory_impulse.weight;
            }
        }
    }

    fn get_bin(&self, measure: f32) -> u8 {
        if measure == 1. {
            self.bins - 1
        } else {
            (measure * (self.bins) as f32).floor() as u8
        }
    }
}

#[derive(Copy, Clone)]
struct FireReceipt {
    fired: bool,
    measure: f32,
}

impl FireReceipt {
    fn new(fired: bool, measure: f32) -> FireReceipt {
        FireReceipt { fired, measure }
    }

    fn new_empty() -> FireReceipt {
        FireReceipt {
            fired: false,
            measure: 0.0,
        }
    }
}

struct FireTracker {
    receipts: (FireReceipt, FireReceipt),
}

impl FireTracker {
    fn new() -> FireTracker {
        FireTracker {
            receipts: (FireReceipt::new_empty(), FireReceipt::new_empty()),
        }
    }

    fn check_receipt(&self, cycle: ChargeCycle) -> FireReceipt {
        match cycle {
            ChargeCycle::Even => self.receipts.0,
            ChargeCycle::Odd => self.receipts.1,
        }
    }

    fn create_receipt(&mut self, cycle: ChargeCycle, fired: bool, measure: f32) {
        match cycle {
            ChargeCycle::Even => {
                self.receipts.0 = FireReceipt::new(fired, measure);
            }
            ChargeCycle::Odd => {
                self.receipts.1 = FireReceipt::new(fired, measure);
            }
        }
    }
}

pub enum SynapseType {
    Excitatory,
    Inhibitory,
}

pub struct Synapse<'a> {
    synapse_type: SynapseType,
    weight: f32,
    target: &'a dyn RxNeuronic,
}

impl Synapse<'_> {
    pub fn new<'a>(synapse_type: SynapseType, weight: f32, target: &'a dyn RxNeuronic) -> Synapse<'a> {
        Synapse {
            synapse_type,
            weight,
            target
        }
    }
}

pub struct SensoryNeuron<'a> {
    measure: RefCell<f32>,
    plastic_synapses: RefCell<Vec<Synapse<'a>>>,
    static_synapses: RefCell<Vec<Synapse<'a>>>,
    weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    fire_tracker: RefCell<FireTracker>,
}

impl SensoryNeuron<'_> {
    pub fn new<'a>(
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    ) -> SensoryNeuron<'a> {
        SensoryNeuron {
            measure: RefCell::new(0.0),
            plastic_synapses: RefCell::new(Vec::new()),
            static_synapses: RefCell::new(Vec::new()),
            weight_modifier,
            fire_tracker: RefCell::new(FireTracker::new()),
        }
    }

    pub fn set_measure(&self, measure: f32) {
        *self.measure.borrow_mut() = measure;
    }
}


impl Neuronic for SensoryNeuron<'_> {
    fn run_cycle(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}

impl TxNeuronic for SensoryNeuron<'_> {
    fn fire_synapses(&self, cycle: ChargeCycle, current_fire_receipt: &FireReceipt, prev_prev_fire_receipt: &FireReceipt) {
        for synapse in self.plastic_synapses.borrow_mut().iter_mut() {
            if current_fire_receipt.fired {
                synapse.target.intake_synaptic_impulse(Impulse::new(synapse.weight, measure));
            }

            let target_receipt = synapse.target.get_fire_receipt(cycle);

            if target_receipt.fired { }
        };
    }

    fn add_plastic_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic) {
        self.plastic_synapses.borrow_mut().push(Synapse::new(synapse_type, weight, target));
    }

    fn add_static_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic) {
        self.static_synapses.borrow_mut().push(Synapse::new(synapse_type, weight, target));
    }
}

pub struct ActuatorNeuron {
    fire_tracker: RefCell<FireTracker>,
    internal_charge: RefCell<InternalCharge>,
    measure: RefCell<f32>,
    fire_threshold: f32,
}

impl ActuatorNeuron {
    pub fn new(charge_bins: u8, fire_threshold: f32) -> ActuatorNeuron {
        ActuatorNeuron {
            fire_tracker: RefCell::new(FireTracker::new()),
            internal_charge: RefCell::new(InternalCharge::new(charge_bins)),
            measure: RefCell::new(0.0),
            fire_threshold,
        }
    }

    pub fn read_measure(&self) -> f32 {
        self.measure.borrow_mut().clone()
    }
}

impl Neuronic for ActuatorNeuron {
    fn run_cycle(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}

impl RxNeuronic for ActuatorNeuron {
    fn intake_synaptic_impulse(&self, impulse: Impulse) {
        unimplemented!()
    }

    fn get_fire_receipt(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}

pub struct PlasticNeuron<'a> {
    fire_tracker: RefCell<FireTracker>,
    internal_charge: RefCell<InternalCharge>,
    plastic_synapses: RefCell<Vec<Synapse<'a>>>,
    static_synapses: RefCell<Vec<Synapse<'a>>>,
    weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    fire_threshold: f32
}

impl PlasticNeuron<'_> {
    pub fn new<'a>(
        charge_bins: u8,
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
        fire_threshold: f32,
    ) -> PlasticNeuron<'a> {
        PlasticNeuron {
            fire_tracker: RefCell::new(FireTracker::new()),
            internal_charge: RefCell::new(InternalCharge::new(charge_bins)),
            plastic_synapses: RefCell::new(Vec::new()),
            static_synapses: RefCell::new(Vec::new()),
            weight_modifier,
            fire_threshold
        }
    }
}

impl Neuronic for PlasticNeuron<'_> {
    fn run_cycle(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}

impl TxNeuronic for PlasticNeuron<'_> {
    fn fire_synapses(&self, measure: f32, cycle: ChargeCycle, fire_receipt: FireReceipt) {
        unimplemented!()
    }

    fn add_plastic_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic) {
        unimplemented!()
    }

    fn add_static_synapse<'a>(&self, weight: f32, synapse_type: SynapseType, target: &'a dyn RxNeuronic) {
        unimplemented!()
    }
}

impl RxNeuronic for PlasticNeuron<'_> {
    fn intake_synaptic_impulse(&self, impulse: Impulse) {
        unimplemented!()
    }

    fn get_fire_receipt(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}


#[cfg(test)]
pub mod neuron_tests;