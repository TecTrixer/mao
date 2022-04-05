use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameState {
    stacks: Vec<Stack>,
    players: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    id: usize,
    state: GameState,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Request {
    Connect { game_id: usize, player_name: String },
    Move { card_id: usize, dest_field: Field },
    Shuffle { stack_id: usize },
    Disconnect { game_id: usize, player_name: String },
}

// TODO: Response

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    id: usize,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Stack {
    PlayStack(Vec<Card>),
    DrawStack(Vec<Card>),
    HandStack(usize, Vec<Card>),
    TableStack(usize, Vec<Card>),
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Field {
    PlayField,
    DrawField,
    HandField(usize),
    TableField(usize),
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Card {
    id: usize,
    field: Field,
    face: CardType,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum CardType {
    HeartTwo,
    HeartThree,
    HeartFour,
    HeartFive,
    HeartSix,
    HeartSeven,
    HeartEight,
    HeartNine,
    HeartTen,
    HeartJack,
    HeartQueen,
    HeartKing,
    HeartAce,
    DiamondTwo,
    DiamondThree,
    DiamondFour,
    DiamondFive,
    DiamondSix,
    DiamondSeven,
    DiamondEight,
    DiamondNine,
    DiamondTen,
    DiamondJack,
    DiamondQueen,
    DiamondKing,
    DiamondAce,
    ClubTwo,
    ClubThree,
    ClubFour,
    ClubFive,
    ClubSix,
    ClubSeven,
    ClubEight,
    ClubNine,
    ClubTen,
    ClubJack,
    ClubQueen,
    ClubKing,
    ClubAce,
    SpadeTwo,
    SpadeThree,
    SpadeFour,
    SpadeFive,
    SpadeSix,
    SpadeSeven,
    SpadeEight,
    SpadeNine,
    SpadeTen,
    SpadeJack,
    SpadeQueen,
    SpadeKing,
    SpadeAce,
    Joker,
}
