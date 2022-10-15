use std::io::{Error, ErrorKind, Read, Write};

use incrementalmerkletree::Hashable;

use crate::{Decodable, Encodable};

impl Encodable for incrementalmerkletree::Position {
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        u64::from(*self).encode(&mut s)
    }
}

impl Decodable for incrementalmerkletree::Position {
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let dec: u64 = Decodable::decode(&mut d)?;
        Ok(Self::try_from(dec).unwrap())
    }
}

impl<T: Encodable + Ord> Encodable for incrementalmerkletree::bridgetree::Leaf<T> {
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;

        match self {
            incrementalmerkletree::bridgetree::Leaf::Left(a) => {
                len += false.encode(&mut s)?;
                len += a.encode(&mut s)?;
            }

            incrementalmerkletree::bridgetree::Leaf::Right(a, b) => {
                len += true.encode(&mut s)?;
                len += a.encode(&mut s)?;
                len += b.encode(&mut s)?;
            }
        }

        Ok(len)
    }
}

impl<T: Decodable + Ord> Decodable for incrementalmerkletree::bridgetree::Leaf<T> {
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let side: bool = Decodable::decode(&mut d)?;

        match side {
            false => {
                let a: T = Decodable::decode(&mut d)?;
                Ok(Self::Left(a))
            }
            true => {
                let a: T = Decodable::decode(&mut d)?;
                let b: T = Decodable::decode(&mut d)?;
                Ok(Self::Right(a, b))
            }
        }
    }
}

impl Encodable for incrementalmerkletree::bridgetree::Checkpoint {
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.bridges_len().encode(&mut s)?;
        len += self.is_witnessed().encode(&mut s)?;
        len += self.witnessed().encode(&mut s)?;
        len += self.forgotten().encode(&mut s)?;
        Ok(len)
    }
}

impl Decodable for incrementalmerkletree::bridgetree::Checkpoint {
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let bridges_len = Decodable::decode(&mut d)?;
        let is_witnessed = Decodable::decode(&mut d)?;
        let witnessed = Decodable::decode(&mut d)?;
        let forgotten = Decodable::decode(&mut d)?;
        Ok(Self::from_parts(bridges_len, is_witnessed, witnessed, forgotten))
    }
}

impl<T: Encodable + Ord + Clone> Encodable
    for incrementalmerkletree::bridgetree::NonEmptyFrontier<T>
{
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.position().encode(&mut s)?;
        len += self.leaf().encode(&mut s)?;
        len += self.ommers().to_vec().encode(&mut s)?;
        Ok(len)
    }
}

impl<T: Decodable + Ord + Clone> Decodable
    for incrementalmerkletree::bridgetree::NonEmptyFrontier<T>
{
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let position = Decodable::decode(&mut d)?;
        let leaf = Decodable::decode(&mut d)?;
        let ommers = Decodable::decode(&mut d)?;

        match Self::from_parts(position, leaf, ommers) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::Other, "FrontierError")),
        }
    }
}

impl<T: Encodable + Ord + Clone> Encodable for incrementalmerkletree::bridgetree::AuthFragment<T> {
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.position().encode(&mut s)?;
        len += self.altitudes_observed().encode(&mut s)?;
        len += self.values().to_vec().encode(&mut s)?;
        Ok(len)
    }
}

impl<T: Decodable + Ord + Clone> Decodable for incrementalmerkletree::bridgetree::AuthFragment<T> {
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let position = Decodable::decode(&mut d)?;
        let altitudes_observed = Decodable::decode(&mut d)?;
        let values = Decodable::decode(&mut d)?;
        Ok(Self::from_parts(position, altitudes_observed, values))
    }
}

impl<T: Encodable + Ord + Clone> Encodable for incrementalmerkletree::bridgetree::MerkleBridge<T> {
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.prior_position().encode(&mut s)?;
        len += self.auth_fragments().encode(&mut s)?;
        len += self.frontier().encode(&mut s)?;
        Ok(len)
    }
}

impl<T: Decodable + Ord + Clone> Decodable for incrementalmerkletree::bridgetree::MerkleBridge<T> {
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let prior_position = Decodable::decode(&mut d)?;
        let auth_fragments = Decodable::decode(&mut d)?;
        let frontier = Decodable::decode(&mut d)?;
        Ok(Self::from_parts(prior_position, auth_fragments, frontier))
    }
}

impl<T: Encodable + Ord + Clone, const V: u8> Encodable
    for incrementalmerkletree::bridgetree::BridgeTree<T, V>
{
    fn encode<S: Write>(&self, mut s: S) -> Result<usize, Error> {
        let mut len = 0;
        len += self.prior_bridges().to_vec().encode(&mut s)?;
        len += self.current_bridge().encode(&mut s)?;
        len += self.witnessed_indices().encode(&mut s)?;
        len += self.checkpoints().to_vec().encode(&mut s)?;
        len += self.max_checkpoints().encode(&mut s)?;
        Ok(len)
    }
}

impl<T: Decodable + Ord + Clone + Hashable, const V: u8> Decodable
    for incrementalmerkletree::bridgetree::BridgeTree<T, V>
{
    fn decode<D: Read>(mut d: D) -> Result<Self, Error> {
        let prior_bridges = Decodable::decode(&mut d)?;
        let current_bridge = Decodable::decode(&mut d)?;
        let saved = Decodable::decode(&mut d)?;
        let checkpoints = Decodable::decode(&mut d)?;
        let max_checkpoints = Decodable::decode(&mut d)?;
        match Self::from_parts(prior_bridges, current_bridge, saved, checkpoints, max_checkpoints) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::Other, "BridgeTreeError")),
        }
    }
}
