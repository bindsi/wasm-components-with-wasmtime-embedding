#[allow(warnings)]
mod bindings;

use bindings::exports::data::validator::unit_validator::Guest;

struct Component;

impl Guest for Component {
    // Executed by host application or another component (in composable mode) which is executing this wasm component.
    fn validate_bar(val: f32) -> bool {
        (0.0..=100.0).contains(&val)
    }

    fn validate_psi(val: f32) -> bool {
        (0.0..=50.0).contains(&val)
    }
}

bindings::export!(Component with_types_in bindings);
