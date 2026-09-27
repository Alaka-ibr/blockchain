//! Contract limits and basis-point denominators.
//!
//! Centralizes the numeric limits used by the ticketing contract so they
//! have one source of truth instead of scattered magic numbers.

/// Maximum number of tickets accepted by the batch entry points.
pub const MAX_BATCH_SIZE: u32 = 50;

/// Ledgers that must pass between proposing and applying a payment token
/// change (~1 day at 5s/ledger).
pub const PAYMENT_TOKEN_CHANGE_DELAY_LEDGERS: u32 = 17_280;

/// Basis-point denominator: 10_000 bps == 100%.
pub const BPS_DENOMINATOR: u32 = 10_000;

/// Stable lowercase labels for the optional common category vocabulary.
pub const CATEGORY_CONCERT: &str = "concert";
pub const CATEGORY_FLIGHT: &str = "flight";
pub const CATEGORY_SPORTS: &str = "sports";
pub const CATEGORY_FESTIVAL: &str = "festival";
pub const CATEGORY_CONFERENCE: &str = "conference";
pub const CATEGORY_OTHER: &str = "other";
