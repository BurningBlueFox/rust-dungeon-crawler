#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    NpcsTurn,
}
