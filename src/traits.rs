use usiagent::shogi::{MochigomaCollections, Teban, Banmen};
use crate::error::ReadError;

pub trait Reader<R> {
    fn read_sfen(&mut self, buf:&[u8]) -> Result<(Teban,Banmen,MochigomaCollections),ReadError>;
    fn read_sfen_with_extended(&mut self, buf:Vec<u8>) -> Result<((Teban,Banmen,MochigomaCollections),R),ReadError>;
}
pub trait TryFrom<T> where Self: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
