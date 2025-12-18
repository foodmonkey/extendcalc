// This file contains the code for building the navigation bar
// from the AssembledKeypads

use crate::data_handling::assembled_keypads::AssembledKeypads;
use cosmic::widget::{icon, nav_bar, row, text, tooltip};

// AssembledKeypads contains a Vec of AssembledKeypads
// AssembledKeypad contains a CalcKeypad and a Vec of CalcKeys
//
// we loop through the Vec of AssembledKeypads and for each
// AssembledKeypad we extract the CalcKeypad label for the navbar_model
// which is AssembledKeypad.CalcKetpad.label

pub fn build_navbar(navbar_model: &mut nav_bar::Model, keypads: &AssembledKeypads) {
    for keypad in &keypads.assembled_keypads {
        let temp_icon = icon::from_name("system-run-symbolic");
        temp_icon.tooltip(keypad.keypad.tooltip.tostring());

        navbar_model
            .insert()
            .data(keypad.keypad.id.to_string())
            .text(keypad.keypad.label.to_string())
            //            .icon(icon::from_name("system-run-symbolic"));
            .icon(temp_icon);
    }
    navbar_model.activate_position(0);
}
