use std::rc::Rc;
use aesir::actuator::Actuator;



fn main() {
   let actuator_names = (0..10).collect::<Vec<u8>>().iter()
       .map(|num| format!("{}", num)).collect::<Vec<String>>();

   let mut actuators: Vec<Rc<dyn Actuator>> = Vec::new();

}