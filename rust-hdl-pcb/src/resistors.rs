use crate::bom::Manufacturer;
use crate::capacitors::make_unpolarized_capacitor;
use crate::circuit::{CircuitNode, PartDetails, Resistor};
use crate::designator::{Designator, DesignatorKind};
use crate::epin::{make_passive_pin_pair, EdgeLocation};
use crate::glyph::{make_label, make_line, make_pin};
use crate::smd::SizeCode;
use crate::utils::pin_list;
use crate::glyph::TextJustification::{BottomLeft, TopLeft};

pub type PowerWatt = num_rational::Rational32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResistorKind {
    ThinFilmChip,
    ThickFilmChip,
    MetalFilm,
}

pub fn map_resistance_to_string(value: f64) -> String {
    if value < 1e3 {
        format!("{:.3}", value).replace(".", "R")
    } else if value < 1e6 {
        format!("{:.3}", value / 1e3).replace(".", "K")
    } else {
        format!("{:.3}", value / 1e6).replace(".", "M")
    }
}

pub fn map_resistance_letter_code_to_value(resistance: &str) -> f64 {
    let mut resistance = resistance.to_owned();
    let mut multiplier = 1.0;
    if resistance.contains("K") {
        multiplier = 1.0e3;
        resistance = resistance.replace("K", ".");
    }
    if resistance.contains("M") {
        multiplier = 1.0e6;
        resistance = resistance.replace("M", ".");
    }
    if resistance.contains("R") {
        multiplier = 1.0;
        resistance = resistance.replace("R", ".");
    }
    resistance.parse::<f64>().unwrap() * multiplier
}

fn make_resistor_details(
    label: String,
    manufacturer: Manufacturer,
    description: String,
    size: SizeCode,
) -> PartDetails {
    PartDetails {
        label: label.clone(),
        manufacturer,
        description,
        comment: "".to_string(),
        hide_pin_designators: true,
        pins: pin_list(make_passive_pin_pair()),
        outline: vec![
            make_pin(-100, 0, EdgeLocation::West, 100),
            make_pin(200, 0, EdgeLocation::East, 100),
            make_line(-100, 0, -70, 30),
            make_line(-70, 30, -10, -30),
            make_line(-10, -30, 50, 30),
            make_line(50, 30, 110, -30),
            make_line(110, -30, 170, 30),
            make_line(170, 30, 200, 0),
            make_label(-110, 40, "R?", BottomLeft),
            make_label(-110, -40, &label, TopLeft),
        ],
        suppliers: vec![],
        designator: Designator {
            kind: DesignatorKind::Resistor,
            index: None,
        },
        size,
        schematic_orientation: Default::default()
    }
}

pub fn make_resistor(
    label: String,
    manufacturer: Manufacturer,
    description: String,
    size: SizeCode,
    value_ohms: f64,
    power_watt: PowerWatt,
    tolerance: f64,
    tempco: Option<f64>,
    kind: ResistorKind,
) -> CircuitNode {
    CircuitNode::Resistor(Resistor {
        details: make_resistor_details(label, manufacturer, description, size),
        value_ohms,
        kind,
        power_watt,
        tolerance,
        tempco,
    })
}
