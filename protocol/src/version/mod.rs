pub mod v1_14_4;
pub mod v1_17_1;

/// Trait to obtain packet ID from packet data.
pub trait PacketId {
    /// Get protcol packet ID.
    fn packet_id(&self) -> u8;
}

#[macro_export]
macro_rules! trait_packet_id (
    ($type: ident, $id: expr) => (
        impl PacketId for $type {
            fn packet_id(&self) -> u8 {
                $id
            }
        }
    )
);
