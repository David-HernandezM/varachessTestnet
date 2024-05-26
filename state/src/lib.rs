#![no_std]
use gmeta::metawasm;
use gstd::prelude::*;
use varachess_io::*;

/*#[metawasm]
pub mod metafns {
    pub type State = ChessState;
    /*pub fn get_players(state: State) -> Vec<ChessPlayer> {
        state.players
    }*/
    pub fn get_games(state: State) -> Vec<ChessState> {
        state.games
    }    
}*/