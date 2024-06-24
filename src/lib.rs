#![no_std]
//use gstd::{debug, mem::replace, msg, prelude::*, ActorId};
use gstd::{debug, msg, prelude::*, ActorId};
use varachess_io::*;

static mut CHESS_STATE: Option<ChessState> = None;

#[no_mangle]
extern "C" fn init() {
    msg::reply("INIT started", 0).expect("Error in INIT reply");
    //debug!(" ** Estoy en INIT")
}

#[no_mangle]
extern "C" fn handle() {
    //debug!(" **Starting handle");
    let action: ChessMessageIn = msg::load().expect("Error in msg::load (handle)");
    let message_response:String;
    let mut message_out: Option<ChessMessageOut> = None;
//
    match action {
       ChessMessageIn::StatusGameId(request_game_id)=>{
            //game_id = find_game_status_into_vector(request_game_id);
            if let Some(game_ref)=find_game_into_vector(request_game_id){
                //debug!(" ** Juego encontrado");
                //debug!(" ** game_ref: {:?}",game_ref);
                //debug!(" ** Pasando a valor propio");
                let game=GameStarted{
                    game_id:game_ref.game_id,
                    game_bet:game_ref.game_bet,
                    game_player1:game_ref.game_player1,
                    game_player2:game_ref.game_player2,
                    game_status:game_ref.game_status.clone(),
                };
                //debug!(" ** game propio: {:?}",game);
                //message_response = String::from("Game_id find");
                message_out=Some(ChessMessageOut::ResponseBoardStatus(game));
            }
            else {
                //debug!(" ** Juego NO encontrado");
                message_out = Some(ChessMessageOut::ResponseString(String::from("Game_id Not found")));
            }
       }
       ChessMessageIn::RequestStartGame(RequestGameStart) => {
            let res = find_modify_or_add_game(RequestGameStart);
            match res{
                StartGameReturnCodes::GameWaiting=>{
                    message_response = String::from("Game in waiting");
                }
                StartGameReturnCodes::GameStarted=>{
                    message_response = String::from("Game started OK");
                }
                StartGameReturnCodes::GameAlreadyStarted=>{
                    message_response = String::from("Error, game is already exist");
                }
                StartGameReturnCodes::PlayerError=>{
                    message_response = String::from("Error, same player ID for both players");
                }
            }
           message_out = Some(ChessMessageOut::ResponseString(message_response));
       }
       ChessMessageIn::EndGame(end_game) => {
            let res = end_game_into_vestor(end_game.game_id);
            //debug!(" El resultado es: {:?}",res);
            //unsafe {debug!(" ** Despues del end, CHESS_STATE: {:?}",CHESS_STATE);};
            match res{
                EndGameReturnCodes::GameEndOk=>{
                    match end_game.result_game{
                        ResultEnd::Win=>{
                            message_response=String::from("Game end OK to Win");
                        }
                        ResultEnd::Lose=>{
                            message_response=String::from("Game end OK to Lose");
                        }
                        ResultEnd::Draw=>{
                            message_response=String::from("Game end OK to Draw");
                        }
                    }
                }
                EndGameReturnCodes::GamePrevFinish=>{message_response=String::from("Error, The game had already been finished")}
                EndGameReturnCodes::GameNotFound=>{message_response=String::from("Error, game not found")}
                EndGameReturnCodes::NoGames=>{message_response=String::from("Error, there are no games")}
           }
           message_out = Some(ChessMessageOut::ResponseString(message_response));
       }
   }
   debug!(" ** message_out: {:?}",message_out);
   msg::reply(message_out, 0).expect("Error in reply handle");

}
 
/*
#[no_mangle]
extern "C" fn handle_reply() {}

fn state_mut() -> &'static mut ChessState {
    let _state = unsafe { CHESS_STATE.as_mut() };
    unsafe { _state.unwrap_unchecked() }
}
 */

#[no_mangle]
extern "C" fn state() {
    let chess_state = unsafe {&CHESS_STATE};
    msg::reply(chess_state, 0).expect("Failed to share state");
}

//Function to add games to the games Vector
pub fn add_game_to_vector(game_to_add :&RequestGameStart,player1:ActorId) {
    let chess_state:Option<ChessState> =  unsafe {
        mem::replace(&mut CHESS_STATE,None )
    };

    match chess_state {
        Some(mut chess_state)=>{
            chess_state.add_game(
                game_to_add.game_id,
                game_to_add.player_bet,
                player1,
                player1,
                StatusGame::Waiting,
            );
            unsafe {CHESS_STATE = Some(chess_state);};
        }
        None => {
            unsafe{
                let chess_state = CHESS_STATE.get_or_insert(ChessState{games:Vec::new()});
                chess_state.add_game(
                    game_to_add.game_id,
                    game_to_add.player_bet,
                    player1,
                    player1,
                    StatusGame::Waiting,
                );
            }
        }
    }
}

//Function to find games into the games Vector
pub fn find_game_into_vector<'a>(game_id_to_find: u64) -> Option<&'a GameStarted> {
    unsafe {
        if let Some(chess_state) = &CHESS_STATE {
            if let Some(game) = chess_state.find_game_by_id(game_id_to_find) {
                //debug!("game find");
                Some(game)
            } else {
                //debug!("game not found");
                None
            }
        } else {
            //debug!("Sin juegos");
            None
        }
    }
}

//Function to end games into the games Vector
pub fn end_game_into_vestor(game_id_change: u64) -> EndGameReturnCodes{
    let chess_state:Option<ChessState> =  unsafe {
        mem::replace(&mut CHESS_STATE,None )
    };
    let code_return:EndGameReturnCodes;
    match chess_state{
        Some(mut chess_state)=>{
            if let Some(game) = chess_state.games.iter_mut().find(|game| game.game_id == game_id_change) {
                //debug!(" ** Juego encontrado: {:?}",game_id_change);
                if let StatusGame::Started = game.game_status {
                    game.game_status=StatusGame::Ended;
                  //  debug!(" ** El estatus ha sido cambiado correctamente");
                    code_return = EndGameReturnCodes::GameEndOk;
                }else {
                //    debug!(" ** El juego ya había sido finalizado");
                    code_return = EndGameReturnCodes::GamePrevFinish;
                }
            }else {
              //  debug!(" ** Juego no encontrado: {:?}",game_id_change);
                code_return = EndGameReturnCodes::GameNotFound;
            }    
            unsafe {CHESS_STATE = Some(chess_state)};
        }
        None =>{
            //debug!(" ** No hay juegos");
            code_return = EndGameReturnCodes::NoGames;
        }
    }
    return  code_return;
}

pub fn find_modify_or_add_game(game_to_add:RequestGameStart)-> StartGameReturnCodes{
    unsafe {
        if let Some(chess_state) = CHESS_STATE.as_mut() {
            if let Some(game) = chess_state.games.iter_mut().find(|g| g.game_id == game_to_add.game_id) {
                if game.game_player1 == game_to_add.player1{
                    return StartGameReturnCodes::PlayerError;
                }
                match game.game_status {
                    StatusGame::Ended=>{
                        return StartGameReturnCodes::GameAlreadyStarted;
                    }
                    StatusGame::Started=>{
                        return StartGameReturnCodes::GameAlreadyStarted;
                    }
                    StatusGame::Waiting=>{
                    // El juego fue encontrado, modificamos player2
                        game.game_player2 = game_to_add.player1;
                        game.game_status=StatusGame::Started;
                        //debug!(" ** Juego actualizado con player2 e iniciado: {:?}",game);
                        StartGameReturnCodes::GameStarted
                    }
                }
            } else {
                // El juego no fue encontrado, lo agregamos
                let new_game = GameStarted {
                    game_id: game_to_add.game_id,
                    game_bet: game_to_add.player_bet,
                    game_player1: game_to_add.player1,
                    game_player2: game_to_add.player1,
                    game_status: StatusGame::Waiting,
                };
                chess_state.games.push(new_game);
                //debug!(" ** Juego actualizado con player2 e iniciado: {:?}",chess_state);
                StartGameReturnCodes::GameWaiting
            }
        } 
        else {
            // CHESS_STATE es None
            //debug!(" ** Chess_state is None");
            unsafe{
                let chess_state = CHESS_STATE.get_or_insert(ChessState{games:Vec::new()});
                chess_state.add_game(
                    game_to_add.game_id,
                    game_to_add.player_bet,
                    game_to_add.player1,
                    game_to_add.player1,
                    StatusGame::Waiting,
                );
                //debug!(" ** Juego agregado y vector iniciado: {:?}",chess_state);
            }
            StartGameReturnCodes::GameWaiting
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Debug)]
enum EndGameReturnCodes{
    GameEndOk,
    GamePrevFinish,
    GameNotFound,
    NoGames,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
enum StartGameReturnCodes{
    GameWaiting,
    GameStarted,
    GameAlreadyStarted,
    PlayerError,
}