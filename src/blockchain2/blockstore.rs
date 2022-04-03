use sled::Batch;

use crate::{
    util::serial::{deserialize, serialize, SerialDecodable, SerialEncodable},
    Result,
};

const SLED_BLOCK_TREE: &[u8] = b"_blocks";

// TODO: The block structure should be as follows
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct Block {
    /// Previous block hash (blake3)
    pub st: blake3::Hash,
    /// Slot UID, generated by the beacon
    pub sl: u64,
    /// Transaction hashes (blake3)
    /// The actual transactions are in [`TxStore`]
    pub txs: Vec<blake3::Hash>,
    /// Additional block information
    pub metadata: String,
}

pub struct BlockStore(sled::Tree);

impl BlockStore {
    /// Opens a new or existing blockstore tree given a sled database.
    pub fn new(db: &sled::Db) -> Result<Self> {
        let tree = db.open_tree(SLED_BLOCK_TREE)?;
        Ok(Self(tree))
    }

    /// Insert a vector of [`Block`] into the blockstore.
    /// The blocks are hashed with blake3 and this blockhash is used as
    // the key, where value is the serialized block itself.
    pub fn insert(&self, blocks: Vec<Block>) -> Result<()> {
        let mut batch = Batch::default();
        for i in &blocks {
            let serialized = serialize(i);
            let blockhash = blake3::hash(&serialized);
            batch.insert(blockhash.as_bytes(), serialized);
        }

        self.0.apply_batch(batch)?;

        Ok(())
    }

    /// Fetch given blockhashes from the blockstore.
    /// The resulting vector contains `Option` which is `Some` if the block
    /// was found in the blockstore, and `None`, if it has not.
    pub fn get(&self, blockhashes: Vec<blake3::Hash>) -> Result<Vec<Option<Block>>> {
        let mut ret: Vec<Option<Block>> = Vec::with_capacity(blockhashes.len());

        for i in &blockhashes {
            if let Some(found) = self.0.get(i.as_bytes())? {
                let block = deserialize(&found)?;
                ret.push(Some(block));
            } else {
                ret.push(None);
            }
        }

        Ok(ret)
    }

    /// Check if the blockstore contains a given blockhash.
    pub fn contains(&self, blockhash: blake3::Hash) -> Result<bool> {
        Ok(self.0.contains_key(blockhash.as_bytes())?)
    }

    /// Fetch the first (oldest) block in the tree.
    pub fn get_first(&self) -> Result<Option<(blake3::Hash, Block)>> {
        if let Some(found) = self.0.first()? {
            let hash_bytes: [u8; 32] = found.0.as_ref().try_into().unwrap();
            let block = deserialize(&found.1)?;
            return Ok(Some((hash_bytes.into(), block)))
        }

        Ok(None)
    }

    /// Fetch the last (newest) block in the tree.
    pub fn get_last(&self) -> Result<Option<(blake3::Hash, Block)>> {
        if let Some(found) = self.0.last()? {
            let hash_bytes: [u8; 32] = found.0.as_ref().try_into().unwrap();
            let block = deserialize(&found.1)?;
            return Ok(Some((hash_bytes.into(), block)))
        }

        Ok(None)
    }

    /// Fetch the block and its hash before the provided blockhash, if one exists.
    pub fn get_lt(&self, blockhash: blake3::Hash) -> Result<Option<(blake3::Hash, Block)>> {
        if let Some(found) = self.0.get_lt(blockhash.as_bytes())? {
            let hash_bytes: [u8; 32] = found.0.as_ref().try_into().unwrap();
            let block = deserialize(&found.1)?;
            return Ok(Some((hash_bytes.into(), block)))
        }

        Ok(None)
    }

    /// Fetch the block and its hash after the provided blockhash, if one exists.
    pub fn get_gt(&self, blockhash: blake3::Hash) -> Result<Option<(blake3::Hash, Block)>> {
        if let Some(found) = self.0.get_gt(blockhash.as_bytes())? {
            let hash_bytes: [u8; 32] = found.0.as_ref().try_into().unwrap();
            let block = deserialize(&found.1)?;
            return Ok(Some((hash_bytes.into(), block)))
        }

        Ok(None)
    }
}