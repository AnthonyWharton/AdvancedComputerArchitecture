use isa::operand::Register;
use simulator::state::State;

///////////////////////////////////////////////////////////////////////////////
//// FUNCTIONS

/// Simple Register Printout given a state, nothing fancy.
pub fn simple_draw_state(state: State) {
    println!("State: {}:{:08x} ({})\r
    {:>#04}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}\r
    {:>#04}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}\r
    {:>#04}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}\r
    {:>#04}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}, {:>#03}:{:08x}\r",
             Register::PC, state.register[Register::PC as usize], state.register[Register::PC as usize],
             Register::X0, state.register[Register::X0 as usize],
             Register::X1, state.register[Register::X1 as usize],
             Register::X2, state.register[Register::X2 as usize],
             Register::X3, state.register[Register::X3 as usize],
             Register::X4, state.register[Register::X4 as usize],
             Register::X5, state.register[Register::X5 as usize],
             Register::X6, state.register[Register::X6 as usize],
             Register::X7, state.register[Register::X7 as usize],
             Register::X8, state.register[Register::X8 as usize],
             Register::X9, state.register[Register::X9 as usize],
             Register::X10, state.register[Register::X10 as usize],
             Register::X11, state.register[Register::X11 as usize],
             Register::X12, state.register[Register::X12 as usize],
             Register::X13, state.register[Register::X13 as usize],
             Register::X14, state.register[Register::X14 as usize],
             Register::X15, state.register[Register::X15 as usize],
             Register::X16, state.register[Register::X16 as usize],
             Register::X17, state.register[Register::X17 as usize],
             Register::X18, state.register[Register::X18 as usize],
             Register::X19, state.register[Register::X19 as usize],
             Register::X20, state.register[Register::X20 as usize],
             Register::X21, state.register[Register::X21 as usize],
             Register::X22, state.register[Register::X22 as usize],
             Register::X23, state.register[Register::X23 as usize],
             Register::X24, state.register[Register::X24 as usize],
             Register::X25, state.register[Register::X25 as usize],
             Register::X26, state.register[Register::X26 as usize],
             Register::X27, state.register[Register::X27 as usize],
             Register::X28, state.register[Register::X28 as usize],
             Register::X29, state.register[Register::X29 as usize],
             Register::X30, state.register[Register::X30 as usize],
             Register::X31, state.register[Register::X31 as usize],
    );
}

