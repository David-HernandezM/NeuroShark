#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{ActorId, Vec};
use ping_pong_io::ProgramMetadata;


#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;



    // pub fn info(state: State) -> NFTQueryReply {
    //     NFTQueryReply::NFTInfo {
    //         name: state.token.name.clone(),
    //         symbol: state.token.symbol.clone(),
    //         base_uri: state.token.base_uri,
    //     }
    // }
}