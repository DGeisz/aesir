use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

/// For better documentation of everything, see the eywa library
/// Most of the names here are equivalent

pub trait Neuronic {
    fn run_cycle(&self, cycle: ChargeCycle);


    fn run_static_cycle(&self, cycle: ChargeCycle);

    fn clear(&self);
}

pub trait TxNeuronic {
    fn update_synapses(
        &self,
        cycle: ChargeCycle,
        fire: bool,
        measure: f32,
        prev_prev_fire_receipt: FireReceipt,
    ) {
        //If neither of the following are true, then nothing occurs in the loop, and we're just burning cycles
        if fire || prev_prev_fire_receipt.fired {
            // First do plastic synapses
            for synapse in self.get_plastic_synapses().borrow_mut().iter_mut() {
                // Fire synapse if it's supposed to be fired
                if fire {
                    synapse.target.intake_synaptic_impulse(
                        cycle,
                        Impulse::new(synapse.weight, measure),
                        synapse.synapse_type,
                    );
                }

                let target_receipt = synapse.target.get_fire_receipt(cycle);

                // Modify weight if the receipts indicate a back to back firing occurred
                if target_receipt.fired && prev_prev_fire_receipt.fired {
                    let new_weight = synapse.weight
                        + (self.get_weight_modifier())(
                            target_receipt.measure,
                            prev_prev_fire_receipt.measure,
                        );

                    synapse.weight = if new_weight > 0.0 { new_weight } else { 0.0 };
                }
            }

            // Then static synapses
            for synapse in self.get_static_synapses().borrow_mut().iter_mut() {
                // Fire synapse if it's supposed to be fired
                if fire {
                    synapse.target.intake_synaptic_impulse(
                        cycle,
                        Impulse::new(synapse.weight, measure),
                        synapse.synapse_type,
                    );
                }

                let target_receipt = synapse.target.get_fire_receipt(cycle);

                // Modify weight if the receipts indicate a back to back firing occurred
                if target_receipt.fired && prev_prev_fire_receipt.fired {
                    let new_weight = synapse.weight
                        + (self.get_weight_modifier())(
                            target_receipt.measure,
                            prev_prev_fire_receipt.measure,
                        );

                    synapse.weight = if new_weight > 0.0 { new_weight } else { 0.0 };
                }
            }
        }
    }

    fn get_plastic_synapses(&self) -> RefMut<Vec<Synapse>>;
    fn get_static_synapses(&self) -> RefMut<Vec<Synapse>>;
    fn get_weight_modifier(&self) -> fn(target_measure: f32, synapse_measure: f32) -> f32;

    fn add_plastic_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    );

    fn add_static_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    );
}

pub trait RxNeuronic {
    fn intake_synaptic_impulse(
        &self,
        cycle: ChargeCycle,
        impulse: Impulse,
        synapse_type: SynapseType,
    ) {
        // println!("Yoge {} {}", impulse.measure, impulse.weight);
        match synapse_type {
            SynapseType::Excitatory => self
                .get_internal_charge_mut()
                .incr_next_charge(cycle, impulse),
            SynapseType::Inhibitory => self
                .get_internal_charge_mut()
                .inhibit_next_charge(cycle, impulse),
        }
    }

    fn get_fire_receipt(&self, cycle: ChargeCycle) -> FireReceipt {
        self.get_fire_tracker().check_receipt(cycle.next_cycle())
    }

    fn get_fire_tracker(&self) -> Ref<FireTracker>;

    fn get_internal_charge_mut(&self) -> RefMut<InternalCharge>;
}

/// Here, the impulse measure is always between 0 and 1
#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone)]
pub enum ChargeCycle {
    Even,
    Odd,
}

impl ChargeCycle {
    pub(crate) fn next_cycle(&self) -> ChargeCycle {
        match self {
            ChargeCycle::Even => ChargeCycle::Odd,
            ChargeCycle::Odd => ChargeCycle::Even,
        }
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

                if weights == 0.0 {
                    0.0
                } else {
                    total_weighted_charge / weights
                }
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

                if weights == 0.0 {
                    0.0
                } else {
                    total_weighted_charge / weights
                }
            }
        }
    }

    fn get_weights(&self, cycle: ChargeCycle) -> f32 {
        match cycle {
            ChargeCycle::Even => {
                let mut weights = 0.0;
                for i in 0..self.bins {
                    let weight = self.even_weights.get(&i).unwrap();

                    if *weight > 0.0 {
                        weights += *weight;
                    }
                }

                weights
            }
            ChargeCycle::Odd => {
                let mut weights = 0.0;
                for i in 0..self.bins {
                    let weight = self.odd_weights.get(&i).unwrap();

                    if *weight > 0.0 {
                        weights += *weight;
                    }
                }

                weights
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
        let mut bin = (measure * (self.bins) as f32).floor() as u8;

        if bin >= self.bins {
            bin = self.bins - 1;
        };

        bin
    }
}

#[derive(Copy, Clone)]
pub struct FireReceipt {
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

pub struct FireTracker {
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

    fn clear_receipts(&mut self) {
        self.receipts = (FireReceipt::new_empty(), FireReceipt::new_empty());
    }
}

#[derive(Copy, Clone)]
pub enum SynapseType {
    Excitatory,
    Inhibitory,
}

pub struct Synapse {
    synapse_type: SynapseType,
    weight: f32,
    target: Rc<dyn RxNeuronic>,
}

impl Synapse {
    pub fn new(synapse_type: SynapseType, weight: f32, target: Rc<dyn RxNeuronic>) -> Synapse {
        Synapse {
            synapse_type,
            weight,
            target,
        }
    }
}

pub struct SensoryNeuron {
    measure: RefCell<f32>,
    plastic_synapses: RefCell<Vec<Synapse>>,
    static_synapses: RefCell<Vec<Synapse>>,
    weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    fire_tracker: RefCell<FireTracker>,
}

impl SensoryNeuron {
    pub fn new(
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    ) -> SensoryNeuron {
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

impl Neuronic for SensoryNeuron {
    fn run_cycle(&self, cycle: ChargeCycle) {
        let mut fire_tracker = self.fire_tracker.borrow_mut();
        let measure = self.measure.borrow();

        self.update_synapses(cycle, true, *measure, fire_tracker.check_receipt(cycle));

        fire_tracker.create_receipt(cycle, true, *measure);
    }

    fn run_static_cycle(&self, cycle: ChargeCycle) {
        let mut fire_tracker = self.fire_tracker.borrow_mut();
        let measure = self.measure.borrow();

        self.update_synapses(cycle, true, *measure, FireReceipt::new_empty());

        fire_tracker.create_receipt(cycle, true, *measure);
    }

    fn clear(&self) {
        self.fire_tracker.borrow_mut().clear_receipts();
    }
}

impl TxNeuronic for SensoryNeuron {
    fn get_plastic_synapses(&self) -> RefMut<Vec<Synapse>> {
        self.plastic_synapses.borrow_mut()
    }

    fn get_static_synapses(&self) -> RefMut<Vec<Synapse>> {
        self.static_synapses.borrow_mut()
    }

    fn get_weight_modifier(&self) -> fn(f32, f32) -> f32 {
        self.weight_modifier
    }

    fn add_plastic_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    ) {
        self.plastic_synapses
            .borrow_mut()
            .push(Synapse::new(synapse_type, weight, target));
    }

    fn add_static_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    ) {
        self.static_synapses
            .borrow_mut()
            .push(Synapse::new(synapse_type, weight, target));
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
        self.measure.borrow().clone()
    }
}

impl Neuronic for ActuatorNeuron {
    fn run_cycle(&self, cycle: ChargeCycle) {
        let mut internal_charge = self.internal_charge.borrow_mut();
        let mut fire_tracker = self.fire_tracker.borrow_mut();

        let weights = internal_charge.get_weights(cycle);

        if weights > self.fire_threshold {
            let measure = internal_charge.get_charge_weighted_average(cycle);
            *self.measure.borrow_mut() = measure;
            fire_tracker.create_receipt(cycle, true, measure);
        } else {
            fire_tracker.create_receipt(cycle, false, 0.0);
        }

        internal_charge.reset_charge(cycle);
    }

    fn run_static_cycle(&self, cycle: ChargeCycle) {
        self.run_cycle(cycle);
    }

    fn clear(&self) {
        let mut internal_charge = self.internal_charge.borrow_mut();

        internal_charge.reset_charge(ChargeCycle::Even);
        internal_charge.reset_charge(ChargeCycle::Odd);

        self.fire_tracker.borrow_mut().clear_receipts();
    }
}

impl RxNeuronic for ActuatorNeuron {
    fn get_fire_tracker(&self) -> Ref<FireTracker> {
        self.fire_tracker.borrow()
    }

    fn get_internal_charge_mut(&self) -> RefMut<InternalCharge> {
        self.internal_charge.borrow_mut()
    }
}

pub struct PlasticNeuron {
    fire_tracker: RefCell<FireTracker>,
    internal_charge: RefCell<InternalCharge>,
    plastic_synapses: RefCell<Vec<Synapse>>,
    static_synapses: RefCell<Vec<Synapse>>,
    weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
    fire_threshold: f32,
}

impl PlasticNeuron {
    pub fn new(
        charge_bins: u8,
        weight_modifier: fn(target_measure: f32, synapse_measure: f32) -> f32,
        fire_threshold: f32,
    ) -> PlasticNeuron {
        PlasticNeuron {
            fire_tracker: RefCell::new(FireTracker::new()),
            internal_charge: RefCell::new(InternalCharge::new(charge_bins)),
            plastic_synapses: RefCell::new(Vec::new()),
            static_synapses: RefCell::new(Vec::new()),
            weight_modifier,
            fire_threshold,
        }
    }
}

impl Neuronic for PlasticNeuron {
    fn run_cycle(&self, cycle: ChargeCycle) {
        let mut fire_tracker = self.fire_tracker.borrow_mut();
        let mut internal_charge = self.internal_charge.borrow_mut();

        let weights = internal_charge.get_weights(cycle);

        if weights > self.fire_threshold {
            let measure = internal_charge.get_charge_weighted_average(cycle);
            self.update_synapses(cycle, true, measure, fire_tracker.check_receipt(cycle));
            fire_tracker.create_receipt(cycle, true, measure);
        } else {
            self.update_synapses(cycle, false, 0.0, fire_tracker.check_receipt(cycle));
            fire_tracker.create_receipt(cycle, false, 0.0);
        }

        internal_charge.reset_charge(cycle);
    }

    fn run_static_cycle(&self, cycle: ChargeCycle) {
        let mut fire_tracker = self.fire_tracker.borrow_mut();
        let mut internal_charge = self.internal_charge.borrow_mut();

        let weights = internal_charge.get_weights(cycle);

        if weights > self.fire_threshold {
            let measure = internal_charge.get_charge_weighted_average(cycle);
            self.update_synapses(cycle, true, measure, FireReceipt::new_empty());
            fire_tracker.create_receipt(cycle, true, measure);
        } else {
            self.update_synapses(cycle, false, 0.0, FireReceipt::new_empty());
            fire_tracker.create_receipt(cycle, false, 0.0);
        }

        internal_charge.reset_charge(cycle);
    }

    fn clear(&self) {
        let mut internal_charge = self.internal_charge.borrow_mut();

        internal_charge.reset_charge(ChargeCycle::Even);
        internal_charge.reset_charge(ChargeCycle::Odd);

        self.fire_tracker.borrow_mut().clear_receipts();
    }
}

impl TxNeuronic for PlasticNeuron {
    fn get_plastic_synapses(&self) -> RefMut<Vec<Synapse>> {
        self.plastic_synapses.borrow_mut()
    }

    fn get_static_synapses(&self) -> RefMut<Vec<Synapse>> {
        self.static_synapses.borrow_mut()
    }

    fn get_weight_modifier(&self) -> fn(f32, f32) -> f32 {
        self.weight_modifier
    }

    fn add_plastic_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    ) {
        self.plastic_synapses
            .borrow_mut()
            .push(Synapse::new(synapse_type, weight, target));
    }

    fn add_static_synapse(
        &self,
        weight: f32,
        synapse_type: SynapseType,
        target: Rc<dyn RxNeuronic>,
    ) {
        self.static_synapses
            .borrow_mut()
            .push(Synapse::new(synapse_type, weight, target));
    }
}

impl RxNeuronic for PlasticNeuron {
    fn get_fire_tracker(&self) -> Ref<FireTracker> {
        self.fire_tracker.borrow()
    }

    fn get_internal_charge_mut(&self) -> RefMut<InternalCharge> {
        self.internal_charge.borrow_mut()
    }
}

/// A basic weight modifier
pub fn basic_weight_modifier(target_measure: f32, weight_measure: f32) -> f32 {
    let x = (target_measure - weight_measure).abs();
    let denominator = 1. - (-1.5_f32).exp();

    let numerator = (-15. * x).exp() - (-1.5_f32).exp();

    return numerator / denominator;
}

#[cfg(test)]
pub mod neuron_tests;
