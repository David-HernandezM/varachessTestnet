use gstd::ActorId;
use gtest::{Log, Program, System};
use varachess_io::*;
const ACTOR2:u64=3;
const ACTOR3:u64=4;
const ACTOR4:u64=4;
 
#[test]
fn happy_path(){
    let sys = System::new();
    let unit_vara:u128 = 10000000000000;
                   
    let bet_3tvara=3*unit_vara;
    let bet_4tvara=4*unit_vara;
    sys.mint_to(ACTOR2, 1000*unit_vara);
    sys.mint_to(ACTOR3, 1000*unit_vara);
    sys.mint_to(ACTOR4, 1000*unit_vara);

    sys.init_logger();
    let program = Program::current(&sys);
    /* INIT MESSAGE */
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*First RequestGameStart sent to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:3.into(),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game in waiting"));
    //let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    let res=program.send_with_value(3,ChessMessageIn::RequestStartGame(game_started_test), bet_3tvara);
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Second RequestGameStart sent to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:4.into(),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game started OK"));
    //let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    let res=program.send_with_value(4,ChessMessageIn::RequestStartGame(game_started_test), bet_3tvara);
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*****Third RequestGameStart sent to  handle for end game -> GameEnd Win******/
    let result_game_tes: ResultEnd = ResultEnd::Win;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a6")};
    let response_out = ChessMessageOut::ResponseString(String::from("Game end OK to Win"));
    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
}

/*/
#[test]
fn status_gameid(){
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    /* INIT MESSAGE */
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*First RequestGameStart sent to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game in waiting"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Second RequestGameStart sent to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game started OK"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*****Third RequestGameStart sent to  handle for end game -> GameEnd Win******/
    let result_game_tes: ResultEnd = ResultEnd::Win;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a6")};
    let response_out = ChessMessageOut::ResponseString(String::from("Game end OK to Win"));
    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*****Send correct game_id to handle of smart contract ******/
    let game_id_test:u64= 123;
    let game_out_find = GameStarted{
        game_id:123,
        game_bet:30,
        game_player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        game_player2:ActorId::new([4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        game_status:StatusGame::Ended,    
    };
    let response_out = ChessMessageOut::ResponseBoardStatus(game_out_find);
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send incorrect game_id to handle of smart contract 
    let game_id_test:u64= 987;
    let response_out =ChessMessageOut::ResponseString(String::from("Game_id Not found"));
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
}

#[test]
fn error_messages_same_gameid(){
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    /* INIT MESSAGE */
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*First sending RequestGameStart to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game in waiting"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Second RequestGameStart sent*/
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game started OK"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Third RequestGameStart sent*/
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Error, game is already exist"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
}

#[test]
fn error_messages_same_playerid(){
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    /* INIT MESSAGE */
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*First sending RequestGameStart to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game in waiting"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Second sending RequestGameStart with the same playerid */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Error, same player ID for both players"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
}

#[test]
fn error_messages_bad_bet(){
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    /* INIT MESSAGE */
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*First sending RequestGameStart to handle of smart contract (RequestGameStarted) */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player1:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Game in waiting"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    /*******************************************************************************/
    /*Second sending RequestGameStart with the same playerid */
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:40,                                                      //Apuesta del jugador
        player1:ActorId::new([4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let response_out=ChessMessageOut::ResponseString(String::from("Error, Bet not same that bat in gameID"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
}
*/

/* LO DEMAS NO */

/*fn with_message_out() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    //**********************************************************************/
    //Send Message to INIT
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    //Send request balance to HANDLE of smart contract (GameStarted)
    let request_balance_test:u64=10;
    let response_out=ChessMessageOut::ResponseString(String::from("OK"));
    let res=program.send(2,ChessMessageIn::RequestBalance(request_balance_test));
    let expected_log= Log::builder()
        .dest(2)
        .payload(Some(response_out));
        assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send first RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
    let response_out=ChessMessageOut::ResponseString(String::from("Game started OK"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    //Send SECOND RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:456,                                                        //Id del juego
        player_bet:60,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
    let response_out = ChessMessageOut::ResponseString(String::from("Game started OK"));
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));

    //**********************************************************************/
    //Send Win message to handle of smart contract -> GameEnd Win
    let result_game_tes: ResultEnd = ResultEnd::Win;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a6")};
    let response_out = ChessMessageOut::ResponseString(String::from("Game end OK to Win"));
    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send correct game_id to handle of smart contract 
    let game_id_test:u64= 123;
    let game_out_find = GameStarted{
        game_id:123,
        game_bet:30,
        game_player1:ActorId::new([2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        game_player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        game_status:StatusGame::Ended,    
    };
    let response_out = ChessMessageOut::ResponseBoardStatus(game_out_find);
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send incorrect game_id to handle of smart contract 
    let game_id_test:u64= 987;
    let response_out =ChessMessageOut::ResponseString(String::from("Game_id Not found"));
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(Some(response_out));
    assert!(res.contains(&expected_log));
    }
    */

/*
#[test]
fn vara_chess_happypath_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    //**********************************************************************/
    //Send Message to INIT
    let res=program.send(2,());
    let expected_log= Log::builder()
        .dest(2)
        .payload("INIT started");
    assert!(res.contains(&expected_log));
    //Send request balance to HANDLE of smart contract (GameStarted)
    let request_balance_test:u64=10;
    let res=program.send(2,ChessMessageIn::RequestBalance(request_balance_test));
    let expected_log= Log::builder()
        .dest(2)
        .payload("OK");
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send Draw message to handle of smart contract but doesn't exist games
    let result_game_tes: ResultEnd = ResultEnd::Draw;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a6")};
    
    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
    .dest(2)
    .payload(String::from("Error, there are no games"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send first RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game started OK"));
    assert!(res.contains(&expected_log));
    //Send SECOND RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:456,                                                        //Id del juego
        player_bet:60,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game started OK"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send aganin the first RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:123,                                                        //Id del juego
        player_bet:30,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Error, game is already exist"));
    assert!(res.contains(&expected_log));
    //Send again the SECOND RequestGameStart to handle of smart contract (RequestGameStarted)
    let game_started_test: RequestGameStart = RequestGameStart{
        game_id:456,                                                        //Id del juego
        player_bet:60,                                                      //Apuesta del jugador
        player2:ActorId::new([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    };
    let res=program.send(2, ChessMessageIn::RequestStartGame(game_started_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Error, game is already exist"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send Win message to handle of smart contract -> GameEnd Win
    let result_game_tes: ResultEnd = ResultEnd::Win;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a6")};

    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game end OK to Win"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send Lose message to handle of smart contract -> GameEnd Lose
    let result_game_tes: ResultEnd = ResultEnd::Lose;
    let game_end_test: GameEnd = GameEnd { game_id: 123, result_game:result_game_tes,position_end_game:String::from("a7")};
    
    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
    .dest(2)
    .payload(String::from("Error, The game had already been finished"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send Draw message to handle of smart contract -> GameEnd Draw
    let result_game_tes: ResultEnd = ResultEnd::Draw;
    let game_end_test: GameEnd = GameEnd { game_id: 456, result_game:result_game_tes,position_end_game:String::from("a8")};

    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game end OK to Draw"));
    assert!(res.contains(&expected_log));
    //Send Draw message to handle of smart contract -> GameEnd Draw
    let result_game_tes: ResultEnd = ResultEnd::Draw;
    let game_end_test: GameEnd = GameEnd { game_id: 789, result_game:result_game_tes,position_end_game:String::from("a1")};

    let res=program.send(2, ChessMessageIn::EndGame(game_end_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Error, game not found"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send game_id message to handle of smart contract 
    let game_id_test:u64= 123;
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game_id find"));
    assert!(res.contains(&expected_log));
    //**********************************************************************/
    //Send game_id message to handle of smart contract 
    let game_id_test:u64= 987;
    let res=program.send(2, ChessMessageIn::StatusGameId(game_id_test));
    //Evaluate result
    let expected_log=Log::builder()
        .dest(2)
        .payload(String::from("Game_id Not found"));
    assert!(res.contains(&expected_log));
}*/

