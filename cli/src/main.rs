use std::io::Write;
use game_core::action::*;
use game_core::card::*;
use game_core::game::*;
use game_core::event::*;
use game_core::phase::*;
use game_core::suit::*;

fn main() {
    let (mut game, _) = Game::new();
    print_board(&game);

    for line in std::io::stdin().lines() {
        let Ok(line) = line else { break; };
        let input = line.trim();
        if input == "q" { break; }

        let Some(action) = parse(input, &game) else {
            println!("看不懂「{input}」，請輸入手牌編號、r 或 q");
            print_board(&game);
            continue;
        };

        for event in game.apply(action) {
            print_event(&event);
        }
        print_board(&game);
    }
}

fn parse(input: &str, game: &Game) -> Option<Action> {
    if input == "r" {
        return Some(Action::Restart);
    }
    let number: usize = input.parse().ok()?;
    let (id, _) = game.hand().get(number.checked_sub(1)?)?;
    Some(Action::Play(*id))
}

fn print_event(event: &GameEvent) {
    match event {
        GameEvent::Captured { .. } => println!("得牌！"),
        GameEvent::Placed(_) => println!("沒得吃，牌留在桌上。"),
        GameEvent::Rejected => println!("現在不能這樣做。"),
        _ => {}
    }
}

fn print_board(game: &Game) {
    let table: Vec<String> = game.table().iter().map(|(_, card)| card_name(card)).collect();
    let hand: Vec<String> = game
        .hand()
        .iter()
        .enumerate()
        .map(|(index, (_, card))| format!("[{}] {}", index + 1, card_name(card)))
        .collect();

    println!();
    println!("分數：{}", game.score());
    println!("桌上：{}", table.join(" "));
    println!("手牌：{}", hand.join(" "));

    match game.phase() {
        Phase::Playing => print!("出第幾張？（r 重來、q 離開）> "),
        Phase::GameOver => print!("Game Over！（r 重來、q 離開）> "),
    }
    let _ = std::io::stdout().flush();
}

fn card_name(card: &Card) -> String {
    let suit = match card.suit {
        Suit::Spade => "♠",
        Suit::Heart => "♥",
        Suit::Diamond => "♦",
        Suit::Club => "♣",
    };
    let rank = match card.rank {
        1 => "A".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        n => n.to_string(),
    };
    format!("{rank}{suit}")
}
