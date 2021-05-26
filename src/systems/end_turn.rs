use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_health = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::NpcsTurn,
        TurnState::NpcsTurn => TurnState::AwaitingInput,
        TurnState::GameOver => current_state,
    };

    player_health.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });
    *turn_state = new_state;
}
