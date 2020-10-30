use crate::ecp_geometry::{EcpBox, EcpGeometry};

#[test]
fn test_get_next_plastic() {
    let ecp_box = EcpBox::new(27, 10, 10, 7);

    let nex_loc = ecp_box.next_plastic_loc(&vec![0, 0, 0]).unwrap();
    assert_eq!(1, *nex_loc.get(0).unwrap());
    assert_eq!(0, *nex_loc.get(1).unwrap());
    assert_eq!(0, *nex_loc.get(2).unwrap());

    let nex_loc = ecp_box.next_plastic_loc(&vec![2, 0, 0]).unwrap();
    assert_eq!(0, *nex_loc.get(0).unwrap());
    assert_eq!(1, *nex_loc.get(1).unwrap());
    assert_eq!(0, *nex_loc.get(2).unwrap());

    let nex_loc = ecp_box.next_plastic_loc(&vec![2, 2, 0]).unwrap();
    assert_eq!(0, *nex_loc.get(0).unwrap());
    assert_eq!(0, *nex_loc.get(1).unwrap());
    assert_eq!(1, *nex_loc.get(2).unwrap());

    let nex_loc = ecp_box.next_plastic_loc(&vec![2, 2, 2]);
    assert_eq!(None, nex_loc);
}

#[test]
fn test_get_next_actuator() {
    let ecp_box = EcpBox::new(125, 10, 123, 7);

    let nex_loc = ecp_box.next_actuator_loc(&vec![0, 5, 0]).unwrap();
    assert_eq!(1, *nex_loc.get(0).unwrap());
    assert_eq!(5, *nex_loc.get(1).unwrap());
    assert_eq!(0, *nex_loc.get(2).unwrap());

    let nex_loc = ecp_box.next_actuator_loc(&vec![4, 5, 0]).unwrap();
    assert_eq!(0, *nex_loc.get(0).unwrap());
    assert_eq!(5, *nex_loc.get(1).unwrap());
    assert_eq!(1, *nex_loc.get(2).unwrap());

    let nex_loc = ecp_box.next_actuator_loc(&vec![4, 5, 1]);
    assert_eq!(None, nex_loc);
}

#[test]
fn test_if_plastic_synapse_with_actuator() {
    let ecp_box = EcpBox::new(125, 10, 123, 26);

    let (plastic, actuators)
        = ecp_box.get_nearby_rx_neurons(&vec![0, 4, 0]);

    assert_eq!(plastic.len(), 20);
    assert_eq!(actuators.len(), 6);

    let (plastic, actuators)
        = ecp_box.get_nearby_rx_neurons(&vec![0, 4, 2]);
    assert_eq!(plastic.len(), 23);
    assert_eq!(actuators.len(), 3);

    let (plastic, actuators)
        = ecp_box.get_nearby_rx_neurons(&vec![4, 4, 4]);
    assert_eq!(plastic.len(), 26);
    assert_eq!(actuators.len(), 0);
}
