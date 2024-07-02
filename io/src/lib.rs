#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
//use gmeta::{In, InOut, Metadata, Out};
use gmeta::{InOut, Metadata, Out};
use scale_info::TypeInfo;

pub type UserAddress = ActorId;
pub type NoWalletAccount = String;
pub type MessageData = (Option<UserAddress>, Option<NoWalletAccount>);

pub mod signless;
use signless::{
    ContractSignlessAccounts,
    SignlessAccount,
    SignlessError
};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<ChessMessageIn, ChessMessageOut>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<ChessStateQuery, ChessStateReply>;
    // type State = Out<ChessState>;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ChessMessageIn {
    RequestStartGame {
        request_game_start: RequestGameStart,
        message_data: MessageData
    },
    StatusGameId(u64),
    EndGame {
        end_game: GameEnd,
        message_data: MessageData
    },
    BindSignlessDataToAddress {
        user_address: ActorId,
        signless_data: SignlessAccount
    },
    BindSignlessDataToNoWalletAccount {
        no_wallet_account: String,
        signless_data: SignlessAccount
    },
}



#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ChessMessageOut {
    ResponseString(String),
//    ResponseStartGame(String),
//    ResponseStatusGame(StatusGame),
    ResponseBoardStatus(GameStarted),
    SignlessMessage(ContractSinglessMessage),
    InvalidSignlessSession
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ResultEnd{
    Win,
    Lose,
    Draw,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum StatusGame{
    Started,
    Waiting,
    Ended,
}

impl Clone for StatusGame {
    fn clone(&self) -> Self {
        match self {
            StatusGame::Started=>StatusGame::Started,
            StatusGame::Waiting=>StatusGame::Waiting,
            StatusGame::Ended=>StatusGame::Ended,
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct RequestGameStart{
    pub game_id: u64,
    pub player_bet: u128,
    pub player1: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub struct GameStarted{
    pub game_id:u64,
    pub game_bet:u128,
    pub game_player1:ActorId,
    pub game_player2: ActorId,
    pub game_status: StatusGame,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct GameEnd{
    pub game_id:u64,
    pub result_game:ResultEnd,
    pub position_end_game:String,
}

pub struct ChessState {
    pub games: Vec<GameStarted>,
    pub signless_data: ContractSignlessAccounts
}

impl ChessState {
    pub fn find_game_by_id(&self,game_id_to_find:u64) -> Option<&GameStarted> {
        self.games.iter().find(|game|game.game_id == game_id_to_find)
    }
    pub fn add_game(&mut self,game_id:u64,game_bet:u128,game_player1:ActorId,game_player2:ActorId,game_status:StatusGame){
        let new_game = GameStarted{
            game_id,
            game_bet,
            game_player1,
            game_player2,
            game_status,
        };
        self.games.push(new_game);
    }
}


#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ChessStateQuery {
    Games,
    SignlessAccountAddressForAddress(ActorId),
    SignlessAccountAddressForNoWalletAccount(NoWalletAccount),
    SignlessAccountData(ActorId)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ChessStateReply {
    SignlessAccountAddressForAddress(Option<ActorId>),
    SignlessAccountAddressForNoWalletAccount(Option<ActorId>),

    Games(Vec<GameStarted>),
    SignlessAccountAddress(Option<ActorId>),
    SignlessAccountData(Option<SignlessAccount>),
    UserIsNotRegistered
}


#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractSinglessMessage {
    Error(SignlessError),
    SignlessAccountSet
}