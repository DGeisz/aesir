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
    fn fire_synapses(&self);
    fn add_plastic_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic);
    fn add_static_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic);
}

pub trait RxNeuronic {
    fn intake_synaptic_impulse(&self, impulse: Impulse);

    fn get_fire_receipt(&self, cycle: ChargeCycle);
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
                self.even.values().sum::<f32>() / self.even_weights.values().sum::<f32>()
            }
            ChargeCycle::Odd => {
                self.odd.values().sum::<f32>() / self.odd_weights.values().sum::<f32>()
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
        let bin = if incr_charge.measure == 1. {
            self.bins - 1
        } else {
            (incr_charge.measure * (self.bins) as f32).floor() as u8
        };

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
    fn fire_synapses(&self) {
        unimplemented!()
    }

    fn add_plastic_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic) {
        unimplemented!()
    }

    fn add_static_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic) {
        unimplemented!()
    }
}

pub struct ActuatorNeuron {
    fire_tracker: RefCell<FireTracker>,
    internal_charge: RefCell<InternalCharge>,
    measure: RefCell<f32>,
    fire_threshold: f32,
}

impl ActuatorNeuron {
    pub fn new(fire_threshold: f32, charge_bins: u8) -> ActuatorNeuron {
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
}

impl PlasticNeuron<'_> {
    pub fn new<'a>(
        charge_bins: u8,
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    ) -> PlasticNeuron<'a> {
        PlasticNeuron {
            fire_tracker: RefCell::new(FireTracker::new()),
            internal_charge: RefCell::new(InternalCharge::new(charge_bins)),
            plastic_synapses: RefCell::new(Vec::new()),
            static_synapses: RefCell::new(Vec::new()),
            weight_modifier
        }
    }
}

impl Neuronic for PlasticNeuron<'_> {
    fn run_cycle(&self, cycle: ChargeCycle) {
        unimplemented!()
    }
}

impl TxNeuronic for PlasticNeuron<'_> {
    fn fire_synapses(&self) {
        unimplemented!()
    }

    fn add_plastic_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic) {
        unimplemented!()
    }

    fn add_static_synapse<'a>(&self, weight: f32, target: &'a dyn RxNeuronic) {
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
mod tests {
    use crate::neuron::{ChargeCycle, FireTracker, Impulse, InternalCharge};

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

        let weight = 15.;
        let measure = 0.2;
        internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));
        let weight = 14.;
        let measure = 0.15;
        internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

        let weight = 10.;
        let measure = 0.5;
        internal_charge.incr_next_charge(ChargeCycle::Odd, Impulse::new(weight, measure));

        assert_eq!(
            (internal_charge.get_charge_weighted_average(ChargeCycle::Even) * 100.).round() as i32,
            26
        );

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
}
