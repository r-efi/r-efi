/// UEFI GUID Partition Tables
///
/// This module defines the data-types related to the new partition tables
/// introduced with UEFI, replacing the old Master Boot Records (MBRs).

// MSRV(aligned-and-packed): `EFI_PARTITION_ENTRY` is 1-byte aligned. We need
//     `packed(1)` to get this in Rust. Unfortunately, Rust forbids embedding
//     custom-aligned types in packed structures, even though we just want the
//     behavior of embedding builtin aligned types. We can work around this by
//     using a private generic and hiding the type.
#[derive(Clone, Debug)]
#[repr(C, packed(1))]
pub struct PartitionEntry<__PrivateGuid = crate::base::Guid> {
    pub partition_type_guid: __PrivateGuid,
    pub unique_partition_guid: __PrivateGuid,
    pub starting_lba: crate::base::Lba,
    pub ending_lba: crate::base::Lba,
    pub attributes: u64,
    pub partition_name: [crate::base::Char16; 36],
}

derive_into_manually_drop!(PartitionEntry);

#[cfg(test)]
mod test {
    use core::mem;
    use super::*;

    // The spec uses `#pragma pack(1)`, so verify that the layout is properly
    // translated into Rust.
    #[test]
    fn layout() {
        assert_eq!(mem::align_of::<PartitionEntry>(), 1);
        assert_eq!(mem::size_of::<PartitionEntry>(), 128);
    }
}
