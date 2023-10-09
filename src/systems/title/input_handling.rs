pub use legion::*;

use crate::{
    components::{CTransform, Gun, Physics, Player},
    state::GameMode,
};

use crate::title::TitleInputs;

#[system]
#[read_component(CTransform)]
#[write_component(Physics)]
#[write_component(Gun)]
#[read_component(Player)]
pub fn handle_inputs(
    #[resource] title_inputs: &TitleInputs,
    #[resource] transition_to: &mut Option<GameMode>,
) {
    if title_inputs.confirm {
        println!("Confirm!");
        *transition_to = Some(GameMode::Playing);
    }
}
