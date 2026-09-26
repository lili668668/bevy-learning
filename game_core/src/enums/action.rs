use crate::structs::card::CardId;

#[derive(Clone, Copy)]
pub enum Action {
    Play(CardId),
    Restart,
}
