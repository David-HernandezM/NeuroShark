#![no_std]

use gmeta::{Out, InOut, Metadata};
use gstd::{prelude::*, ActorId};


pub type PingStateType = Vec<(ActorId, PingAction)>;
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = Out<String>;
    type Handle = InOut<PingAction, PingEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<PingStateType>;
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PingAction {
    #[default]
    Ping,
    Pong
}

#[derive(Default, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PingEvent {
    #[default]
    Ping,
    Pong
}