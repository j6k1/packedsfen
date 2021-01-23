use usiagent::shogi::{MochigomaCollections, Teban, Banmen};

pub trait Reader<R> {
    fn read_sfen(buf:&[u8]) -> (Teban,Banmen,MochigomaCollections);
    fn read_sfen_with_extended(buf:&[u8]) -> ((Teban,Banmen,MochigomaCollections),R);
}
pub trait TryFrom<T> where Self: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
