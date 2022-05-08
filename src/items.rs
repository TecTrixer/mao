#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameState {
    #[prost(message, repeated, tag="1")]
    pub stacks: ::prost::alloc::vec::Vec<Stack>,
    #[prost(message, repeated, tag="2")]
    pub players: ::prost::alloc::vec::Vec<Player>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Game {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<GameState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Request", tags="1, 2, 3, 4")]
    pub request: ::core::option::Option<request::Request>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        Connect(super::Connect),
        #[prost(message, tag="2")]
        Move(super::Move),
        #[prost(message, tag="3")]
        Shuffle(super::Shuffle),
        #[prost(message, tag="4")]
        State(super::State),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Response", tags="1, 2, 3, 4")]
    pub response: ::core::option::Option<response::Response>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Connected(super::Connected),
        #[prost(message, tag="2")]
        Move(super::Move),
        #[prost(message, tag="3")]
        Shuffle(super::Shuffle),
        #[prost(message, tag="4")]
        State(super::GameState),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connected {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub here: ::core::option::Option<GameState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connect {
    #[prost(int32, tag="1")]
    pub game_id: i32,
    #[prost(string, tag="2")]
    pub player_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Move {
    #[prost(int32, tag="1")]
    pub card_id: i32,
    #[prost(message, optional, tag="2")]
    pub dest_field: ::core::option::Option<Field>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shuffle {
    #[prost(int32, tag="1")]
    pub stack_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(oneof="field::Location", tags="1, 2, 3, 4")]
    pub location: ::core::option::Option<field::Location>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        #[prost(message, tag="1")]
        PlayField(super::PlayField),
        #[prost(message, tag="2")]
        DrawField(super::DrawField),
        #[prost(message, tag="3")]
        HandField(super::HandField),
        #[prost(message, tag="4")]
        TableField(super::TableField),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stack {
    #[prost(oneof="stack::Stack", tags="1, 2, 3, 4")]
    pub stack: ::core::option::Option<stack::Stack>,
}
/// Nested message and enum types in `Stack`.
pub mod stack {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Stack {
        #[prost(message, tag="1")]
        Play(super::Play),
        #[prost(message, tag="2")]
        Draw(super::Draw),
        #[prost(message, tag="3")]
        Hand(super::Hand),
        #[prost(message, tag="4")]
        Table(super::Table),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Play {
    #[prost(message, repeated, tag="1")]
    pub card: ::prost::alloc::vec::Vec<Card>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Draw {
    #[prost(message, repeated, tag="1")]
    pub card: ::prost::alloc::vec::Vec<Card>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hand {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, repeated, tag="2")]
    pub card: ::prost::alloc::vec::Vec<Card>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, repeated, tag="2")]
    pub card: ::prost::alloc::vec::Vec<Card>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayField {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrawField {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandField {
    #[prost(int32, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableField {
    #[prost(int32, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Card {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub field: ::core::option::Option<Field>,
    #[prost(message, optional, tag="3")]
    pub face: ::core::option::Option<CardType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardType {
}
/// Nested message and enum types in `CardType`.
pub mod card_type {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Cards {
        Joker = 0,
        HeartTwo = 1,
        HeartThree = 2,
        HeartFour = 3,
        HeartFive = 4,
        HeartSix = 5,
        HeartSeven = 6,
        HeartEight = 7,
        HeartNine = 8,
        HeartTen = 9,
        HeartJack = 10,
        HeartQueen = 11,
        HeartKing = 12,
        HeartAce = 13,
        DiamondTwo = 14,
        DiamondThree = 15,
        DiamondFour = 16,
        DiamondFive = 17,
        DiamondSix = 18,
        DiamondSeven = 19,
        DiamondEight = 20,
        DiamondNine = 21,
        DiamondTen = 22,
        DiamondJack = 23,
        DiamondQueen = 24,
        DiamondKing = 25,
        DiamondAce = 26,
        ClubTwo = 27,
        ClubThree = 28,
        ClubFour = 29,
        ClubFive = 30,
        ClubSix = 31,
        ClubSeven = 32,
        ClubEight = 33,
        ClubNine = 34,
        ClubTen = 35,
        ClubJack = 36,
        ClubQueen = 37,
        ClubKing = 38,
        ClubAce = 39,
        SpadeTwo = 40,
        SpadeThree = 41,
        SpadeFour = 42,
        SpadeFive = 43,
        SpadeSix = 44,
        SpadeSeven = 45,
        SpadeEight = 46,
        SpadeNine = 47,
        SpadeTen = 48,
        SpadeJack = 49,
        SpadeQueen = 50,
        SpadeKing = 51,
        SpadeAce = 52,
    }
}
