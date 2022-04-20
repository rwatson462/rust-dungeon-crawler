
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TurnState{
    //MainMenu,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver
}