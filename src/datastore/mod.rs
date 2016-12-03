mod traits;
pub use self::traits::*;

// NOTE: We are exporting test code. Ideally this would not be the case, but it appears that
// outside packages cannot access exported things with #[cfg(test)], even when they themselves are
// configured for tests.
#[macro_use]
pub mod tests;
