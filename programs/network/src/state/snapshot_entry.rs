use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT_ENTRY: &[u8] = b"snapshot_entry";

/**
 * SnapshotEntry
 */
#[account]
#[derive(Debug)]
pub struct SnapshotEntry {
    pub bump: u8,
    pub id: u64,
    pub node_identity: Pubkey,
    pub snapshot: Pubkey,
    pub stake_offset: u64,
    pub stake_size: u64,
}

impl SnapshotEntry {
    pub fn pda(snapshot: Pubkey, id: u64) -> PDA {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_ENTRY,
                snapshot.as_ref(),
                id.to_be_bytes().as_ref(),
            ],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for SnapshotEntry {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        SnapshotEntry::try_deserialize(&mut data.as_slice())
    }
}

/**
 * SnapshotEntryAccount
 */

pub trait SnapshotEntryAccount {
    fn new(
        &mut self,
        bump: u8,
        id: u64,
        node_identity: Pubkey,
        stake_offset: u64,
        stake_size: u64,
        snapshot: Pubkey,
    ) -> Result<()>;
}

impl SnapshotEntryAccount for Account<'_, SnapshotEntry> {
    fn new(
        &mut self,
        bump: u8,
        id: u64,
        node_identity: Pubkey,
        stake_offset: u64,
        stake_size: u64,
        snapshot: Pubkey,
    ) -> Result<()> {
        self.bump = bump;
        self.id = id;
        self.node_identity = node_identity;
        self.stake_offset = stake_offset;
        self.stake_size = stake_size;
        self.snapshot = snapshot;
        Ok(())
    }
}
