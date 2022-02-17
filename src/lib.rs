mod blocktime;
pub mod instruction;

pub use blocktime::blocktime;
pub use cronos_program::errors;
pub use cronos_program::pda;
pub use cronos_program::state as account;

// Export current solana-program types for downstream users who may also be
// building with a different solana-program version
pub use solana_program;

// The library uses this to verify the keys
solana_program::declare_id!(cronos_program::ID);
