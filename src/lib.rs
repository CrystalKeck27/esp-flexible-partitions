

#[cfg(not(feature = "std"))]
mod rom;
#[cfg(not(feature = "std"))]
pub(crate) use rom as crypto;

#[cfg(feature = "std")]
mod non_rom;
#[cfg(embedded_test)]
pub use crypto::Crc32 as Crc32ForTesting;
#[cfg(feature = "std")]
pub(crate) use non_rom as crypto;


pub mod partitions;
