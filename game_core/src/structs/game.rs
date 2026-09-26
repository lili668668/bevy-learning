use crate::enums::action::Action;
use crate::structs::card::*;
use crate::enums::event::GameEvent;
use crate::enums::phase::Phase;
use crate::enums::suit::Suit;

pub struct Game {
    hand: Vec<(CardId, Card)>,
    table: Vec<(CardId, Card)>,
    score: u32,
    phase: Phase,
    next_id: u32,
}

impl Game {
    pub fn new() -> (Self, Vec<GameEvent>) {
        let mut game = Game { hand: Vec::new(), table: Vec::new(), score: 0, phase: Phase::Playing, next_id: 0 };
        let events = game.deal();
        (game, events)
    }

    pub fn hand(&self) -> &[(CardId, Card)] { &self.hand }
    pub fn table(&self) -> &[(CardId, Card)] { &self.table }
    pub fn score(&self) -> u32 { self.score }
    pub fn phase(&self) -> Phase { self.phase }

    pub fn apply(&mut self, action: Action) -> Vec<GameEvent> {
        match (self.phase, action) {
            (Phase::Playing, Action::Play(id)) => self.play(id),
            (Phase::GameOver, Action::Restart) => self.deal(),
            _ => vec![GameEvent::Rejected],
        }
    }

    fn deal(&mut self) -> Vec<GameEvent> {
        self.hand.clear();
        self.table.clear();
        self.score = 0;
        self.phase = Phase::Playing;
        for suit in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
            let id = CardId(self.next_id);
            self.next_id += 1;
            self.hand.push((id, Card { suit, rank: 1 }));
        }
        vec![GameEvent::Dealt(self.hand.iter().map(|(id, _)| *id).collect()), GameEvent::ScoreChanged(0)]
    }

    fn play(&mut self, id: CardId) -> Vec<GameEvent> {
        let Some(hand_index) = self.hand.iter().position(|(card_id, _)| *card_id == id) else {
            return vec![GameEvent::Rejected];
        };
        let (_, played) = self.hand.remove(hand_index);

        let mut events = Vec::new();
        match self.table.iter().position(|(_, card)| card.rank == played.rank) {
            Some(table_index) => {
                let (table_card, _) = self.table.remove(table_index);
                self.score += 1;
                events.push(GameEvent::Captured { hand_card: id, table_card });
                events.push(GameEvent::ScoreChanged(self.score));
            }
            None => {
                self.table.push((id, played));
                events.push(GameEvent::Placed(id));
            }
        }

        if self.hand.is_empty() {
            self.phase = Phase::GameOver;
            events.push(GameEvent::GameOver);
        }
        events
    }
}

#[test]
fn full_round_and_restart() {
    let (mut game, events) = Game::new();
    assert_eq!(events, vec![GameEvent::Dealt(vec![CardId(0), CardId(1), CardId(2), CardId(3)]), GameEvent::ScoreChanged(0)]);

    assert_eq!(game.apply(Action::Restart), vec![GameEvent::Rejected]);
    assert_eq!(game.apply(Action::Play(CardId(0))), vec![GameEvent::Placed(CardId(0))]);
    assert_eq!(game.apply(Action::Play(CardId(0))), vec![GameEvent::Rejected]);
    assert_eq!(game.apply(Action::Play(CardId(1))), vec![GameEvent::Captured { hand_card: CardId(1), table_card: CardId(0) }, GameEvent::ScoreChanged(1)]);
    assert_eq!(game.apply(Action::Play(CardId(2))), vec![GameEvent::Placed(CardId(2))]);
    assert_eq!(game.apply(Action::Play(CardId(3))), vec![GameEvent::Captured { hand_card: CardId(3), table_card: CardId(2) }, GameEvent::ScoreChanged(2), GameEvent::GameOver]);
    assert_eq!(game.phase(), Phase::GameOver);
    assert_eq!(game.apply(Action::Play(CardId(0))), vec![GameEvent::Rejected]);

    let events = game.apply(Action::Restart);
    assert_eq!(events, vec![GameEvent::Dealt(vec![CardId(4), CardId(5), CardId(6), CardId(7)]), GameEvent::ScoreChanged(0)]);
    assert_eq!((game.score(), game.hand().len(), game.table().len(), game.phase()), (0, 4, 0, Phase::Playing));
}
