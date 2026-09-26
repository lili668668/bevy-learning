use crate::card::CardId;

#[derive(PartialEq, Eq, Debug)]
pub enum GameEvent {
    Dealt(Vec<CardId>),
    Placed(CardId),
    Captured { hand_card: CardId, table_card: CardId },
    ScoreChanged(u32),
    GameOver,
    Rejected,
}
