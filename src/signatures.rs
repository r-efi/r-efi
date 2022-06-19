pub mod system {
    pub type EventNotifySignature = eficall! {fn(crate::base::Event, *mut core::ffi::c_void)};

    pub mod runtime_services {
        use crate::system;

        pub type GetTimeSignature = eficall! {fn(
            *mut system::Time,
            *mut system::TimeCapabilities,
        ) -> crate::base::Status};
        pub type SetTimeSignature = eficall! {fn(
            *mut system::Time,
        ) -> crate::base::Status};
        pub type GetWakeupTimeSignature = eficall! {fn(
            *mut crate::base::Boolean,
            *mut crate::base::Boolean,
            *mut system::Time,
        ) -> crate::base::Status};
        pub type SetWakeupTimeSignature = eficall! {fn(
            crate::base::Boolean,
            *mut system::Time,
        ) -> crate::base::Status};

        pub type SetVirtualAddressMapSignature = eficall! {fn(
            usize,
            usize,
            u32,
            *mut system::MemoryDescriptor,
        ) -> crate::base::Status};
        pub type ConvertPointerSignature = eficall! {fn(
            usize,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};

        pub type GetVariableSignature = eficall! {fn(
            *mut crate::base::Char16,
            *mut crate::base::Guid,
            *mut u32,
            *mut usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type GetNextVariableNameSignature = eficall! {fn(
            *mut usize,
            *mut crate::base::Char16,
            *mut crate::base::Guid,
        ) -> crate::base::Status};
        pub type SetVariableSignature = eficall! {fn(
            *mut crate::base::Char16,
            *mut crate::base::Guid,
            u32,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};

        pub type GetNextHighMonoCountSignature = eficall! {fn(
            *mut u32,
        ) -> crate::base::Status};
        pub type ResetSystemSignature = eficall! {fn(
            system::ResetType,
            crate::base::Status,
            usize,
            *mut core::ffi::c_void,
        )};

        pub type UpdateCapsuleSignature = eficall! {fn(
            *mut *mut system::CapsuleHeader,
            usize,
            crate::base::PhysicalAddress,
        ) -> crate::base::Status};
        pub type QueryCapsuleCapabilitiesSignature = eficall! {fn(
            *mut *mut system::CapsuleHeader,
            usize,
            *mut u64,
            *mut system::ResetType,
        ) -> crate::base::Status};
        pub type QueryVariableInfoSignature = eficall! {fn(
            u32,
            *mut u64,
            *mut u64,
            *mut u64,
        ) -> crate::base::Status};
    }

    pub mod boot_services {
        use crate::system;

        pub type RaiseTplSignature = eficall! {fn(
            crate::base::Tpl,
        ) -> crate::base::Tpl};
        pub type RestoreTplSignature = eficall! {fn(
            crate::base::Tpl,
        )};

        pub type AllocatePagesSignature = eficall! {fn(
            system::AllocateType,
            system::MemoryType,
            usize,
            *mut crate::base::PhysicalAddress,
        ) -> crate::base::Status};
        pub type FreePagesSignature = eficall! {fn(
            crate::base::PhysicalAddress,
            usize,
        ) -> crate::base::Status};
        pub type GetMemoryMapSignature = eficall! {fn(
            *mut usize,
            *mut system::MemoryDescriptor,
            *mut usize,
            *mut usize,
            *mut u32,
        ) -> crate::base::Status};
        pub type AllocatePoolSignature = eficall! {fn(
            system::MemoryType,
            usize,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type FreePoolSignature = eficall! {fn(
            *mut core::ffi::c_void,
        ) -> crate::base::Status};

        pub type CreateEventSignature = eficall! {fn(
            u32,
            crate::base::Tpl,
            system::EventNotify,
            *mut core::ffi::c_void,
            *mut crate::base::Event,
        ) -> crate::base::Status};
        pub type SetTimerSignature = eficall! {fn(
            crate::base::Event,
            system::TimerDelay,
            u64,
        ) -> crate::base::Status};
        pub type WaitForEventSignature = eficall! {fn(
            usize,
            *mut crate::base::Event,
            *mut usize,
        ) -> crate::base::Status};
        pub type SignalEventSignature = eficall! {fn(
            crate::base::Event,
        ) -> crate::base::Status};
        pub type CloseEventSignature = eficall! {fn(
            crate::base::Event,
        ) -> crate::base::Status};
        pub type CheckEventSignature = eficall! {fn(
            crate::base::Event,
        ) -> crate::base::Status};

        pub type InstallProtocolInterfaceSignature = eficall! {fn(
            *mut crate::base::Handle,
            *mut crate::base::Guid,
            system::InterfaceType,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type ReinstallProtocolInterfaceSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type UninstallProtocolInterfaceSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type HandleProtocolSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type RegisterProtocolNotifySignature = eficall! {fn(
            *mut crate::base::Guid,
            crate::base::Event,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type LocateHandleSignature = eficall! {fn(
            system::LocateSearchType,
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
            *mut usize,
            *mut crate::base::Handle,
        ) -> crate::base::Status};
        pub type LocateDevicePathSignature = eficall! {fn(
            *mut crate::base::Guid,
            *mut *mut crate::protocols::device_path::Protocol,
            *mut crate::base::Handle,
        ) -> crate::base::Status};

        pub type InstallConfigurationTableSignature = eficall! {fn(
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};

        pub type LoadImageSignature = eficall! {fn(
            crate::base::Boolean,
            crate::base::Handle,
            *mut crate::protocols::device_path::Protocol,
            *mut core::ffi::c_void,
            usize,
            *mut crate::base::Handle,
        ) -> crate::base::Status};
        pub type StartImageSignature = eficall! {fn(
            crate::base::Handle,
            *mut usize,
            *mut *mut crate::base::Char16,
        ) -> crate::base::Status};
        pub type ExitSignature = eficall! {fn(
            crate::base::Handle,
            crate::base::Status,
            usize,
            *mut crate::base::Char16,
        ) -> crate::base::Status};
        pub type UnloadImageSignature = eficall! {fn(
            crate::base::Handle,
        ) -> crate::base::Status};
        pub type ExitBootServicesSignature = eficall! {fn(
            crate::base::Handle,
            usize,
        ) -> crate::base::Status};

        pub type GetNextMonotonicCountSignature = eficall! {fn(
            *mut u64,
        ) -> crate::base::Status};
        pub type StallSignature = eficall! {fn(
            usize,
        ) -> crate::base::Status};
        pub type SetWatchdogTimerSignature = eficall! {fn(
            usize,
            u64,
            usize,
            *mut crate::base::Char16,
        ) -> crate::base::Status};

        pub type ConnectControllerSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Handle,
            *mut crate::protocols::device_path::Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type DisconnectControllerSignature = eficall! {fn(
            crate::base::Handle,
            crate::base::Handle,
            crate::base::Handle,
        ) -> crate::base::Status};

        pub type OpenProtocolSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            *mut *mut core::ffi::c_void,
            crate::base::Handle,
            crate::base::Handle,
            u32,
        ) -> crate::base::Status};
        pub type CloseProtocolSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            crate::base::Handle,
            crate::base::Handle,
        ) -> crate::base::Status};
        pub type OpenProtocolInformationSignature = eficall! {fn(
            crate::base::Handle,
            *mut crate::base::Guid,
            *mut *mut system::OpenProtocolInformationEntry,
            *mut usize,
        ) -> crate::base::Status};

        pub type ProtocolsPerHandleSignature = eficall! {fn(
            crate::base::Handle,
            *mut *mut *mut crate::base::Guid,
            *mut usize,
        ) -> crate::base::Status};
        pub type LocateHandleBufferSignature = eficall! {fn(
            system::LocateSearchType,
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
            *mut usize,
            *mut *mut crate::base::Handle,
        ) -> crate::base::Status};
        pub type LocateProtocolSignature = eficall! {fn(
            *mut crate::base::Guid,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type InstallMultipleProtocolInterfacesSignature = eficall! {fn(
            *mut crate::base::Handle,
            // XXX: Actual definition is variadic. See eficall!{} for details.
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type UninstallMultipleProtocolInterfacesSignature = eficall! {fn(
            *mut crate::base::Handle,
            // XXX: Actual definition is variadic. See eficall!{} for details.
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};

        pub type CalculateCRC32Signature = eficall! {fn(
            *mut core::ffi::c_void,
            usize,
            *mut u32,
        ) -> crate::base::Status};

        pub type CopyMemSignature = eficall! {fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            usize,
        )};
        pub type SetMemSignature = eficall! {fn(
            *mut core::ffi::c_void,
            usize,
            u8,
        )};

        // 2.0+
        pub type CreateEventExSignature = eficall! {fn(
            u32,
            crate::base::Tpl,
            system::EventNotify,
            *const core::ffi::c_void,
            *const crate::base::Guid,
            *mut crate::base::Event,
        ) -> crate::base::Status};
    }
}

pub mod protocols {
    pub mod simple_text_output {
        use crate::protocols::simple_text_output::Protocol;

        pub type ResetSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type OutputStringSignature = eficall! {fn(
            *mut Protocol,
            *mut crate::base::Char16,
        ) -> crate::base::Status};
        pub type TestStringSignature = eficall! {fn(
            *mut Protocol,
            *mut crate::base::Char16,
        ) -> crate::base::Status};
        pub type QueryModeSignature = eficall! {fn(
            *mut Protocol,
            usize,
            *mut usize,
            *mut usize,
        ) -> crate::base::Status};
        pub type SetModeSignature = eficall! {fn(
            *mut Protocol,
            usize,
        ) -> crate::base::Status};
        pub type SetAttributeSignature = eficall! {fn(
            *mut Protocol,
            usize,
        ) -> crate::base::Status};
        pub type ClearScreenSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type SetCursorPositionSignature = eficall! {fn(
            *mut Protocol,
            usize,
            usize,
        ) -> crate::base::Status};
        pub type EnableCursorSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
    }

    pub mod simple_text_input {
        use crate::protocols::simple_text_input::{InputKey, Protocol};

        pub type ResetSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type ReadKeyStorkeSignature = eficall! {fn(
            *mut Protocol,
            *mut InputKey,
        ) -> crate::base::Status};
    }

    pub mod simple_text_input_ex {
        use crate::protocols::simple_text_input_ex::{
            KeyData, KeyNotifyFunction, KeyToggleState, Protocol,
        };

        pub type KeyNotifyFunctionSignature = eficall! {fn(*mut KeyData) -> crate::base::Status};

        pub type ResetSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type ReadKeyStrokeExSignature = eficall! {fn(
            *mut Protocol,
            *mut KeyData,
        ) -> crate::base::Status};
        pub type SetStateSignature = eficall! {fn(
            *mut Protocol,
            *mut KeyToggleState,
        ) -> crate::base::Status};
        pub type RegisterKeyNotifySignature = eficall! {fn(
            *mut Protocol,
            *mut KeyData,
            KeyNotifyFunction,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type UnregisterKeyNotifySignature = eficall! {fn(
            *mut Protocol,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
    }

    pub mod simple_network {
        use crate::protocols::simple_network::{Protocol, Statistics};

        pub type StartSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type StopSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type InitializeSignature = eficall! {fn(
            *mut Protocol,
            usize,
            usize,
        ) -> crate::base::Status};
        pub type ResetSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type ShutdownSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type ReceiveFiltersSignature = eficall! {fn(
            *mut Protocol,
            u32,
            u32,
            crate::base::Boolean,
            usize,
            *mut crate::base::MacAddress,
        ) -> crate::base::Status};
        pub type StationAddressSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
            *mut crate::base::MacAddress,
        ) -> crate::base::Status};
        pub type StatisticsSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
            *mut usize,
            *mut Statistics,
        ) -> crate::base::Status};
        pub type McastIpToMacSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
            *mut crate::base::IpAddress,
            *mut crate::base::MacAddress,
        ) -> crate::base::Status};
        pub type NvDataSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
            usize,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type GetStatusSignature = eficall! {fn(
            *mut Protocol,
            *mut u32,
            *mut *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type TransmitSignature = eficall! {fn(
            *mut Protocol,
            usize,
            usize,
            *mut core::ffi::c_void,
            *mut crate::base::MacAddress,
            *mut crate::base::MacAddress,
            *mut u16,
        ) -> crate::base::Status};
        pub type ReceiveSignature = eficall! {fn(
            *mut Protocol,
            *mut usize,
            *mut usize,
            *mut core::ffi::c_void,
            *mut crate::base::MacAddress,
            *mut crate::base::MacAddress,
            *mut u16,
        ) -> crate::base::Status};
    }

    pub mod block_io {
        use crate::protocols::block_io::Protocol;

        pub type ResetSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Boolean,
        ) -> crate::base::Status};
        pub type ReadBlocksSignature = eficall! {fn(
            *mut Protocol,
            u32,
            crate::base::Lba,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type WriteBlocksSignature = eficall! {fn(
            *mut Protocol,
            u32,
            crate::base::Lba,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type FlushBlocksSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
    }

    pub mod simple_file_system {
        use crate::protocols::simple_file_system::Protocol;

        pub type OpenVolumeSignature = eficall! {fn(
            *mut Protocol,
            *mut *mut crate::protocols::file::Protocol,
        ) -> crate::base::Status};
    }

    pub mod loaded_image {
        pub type UnloadSignature = eficall! {fn(
            crate::base::Handle,
        ) -> crate::base::Status};
    }

    pub mod hii_string {
        use crate::protocols::hii_string::{Info, Protocol};

        pub type NewStringSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *mut crate::hii::StringId,
            *const crate::base::Char8,
            *const crate::base::Char16,
            crate::protocols::hii_font::String,
            *const Info,
        ) -> crate::base::Status};
        pub type GetStringSignature = eficall! {fn(
            *const Protocol,
            *const crate::base::Char8,
            crate::hii::Handle,
            crate::hii::StringId,
            crate::protocols::hii_font::String,
            *mut usize,
            *mut *mut Info,
        ) -> crate::base::Status};
        pub type SetStringSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            crate::hii::StringId,
            *const crate::base::Char8,
            crate::protocols::hii_font::String,
            *const Info,
        ) -> crate::base::Status};
        pub type GetLanguagesSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *mut crate::base::Char8,
            *mut usize,
        ) -> crate::base::Status};
        pub type GetSecondaryLanguagesSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *const crate::base::Char8,
            *mut crate::base::Char8,
            *mut usize,
        ) -> crate::base::Status};
    }

    pub mod hii_font_ex {
        use crate::protocols::hii_font_ex::{DisplayInfo, ImageOutput, Protocol};

        pub type StringToImageExSignature = eficall! {fn(
            *const Protocol,
            crate::protocols::hii_font::OutFlags,
            crate::protocols::hii_font::String,
            *const DisplayInfo,
            *mut *mut ImageOutput,
            usize,
            usize,
            *mut *mut crate::protocols::hii_font::RowInfo,
            *mut usize,
            *mut usize,
        ) -> crate::base::Status};
        pub type StringIdToImageExSignature = eficall! {fn(
            *const Protocol,
            crate::protocols::hii_font::OutFlags,
            crate::hii::Handle,
            crate::hii::StringId,
            *const crate::base::Char8,
            *const DisplayInfo,
            *mut *mut ImageOutput,
            usize,
            usize,
            *mut *mut crate::protocols::hii_font::RowInfo,
            *mut usize,
            *mut usize,
        ) -> crate::base::Status};
        pub type GetGlyphExSignature = eficall! {fn(
            *const Protocol,
            crate::base::Char16,
            *const DisplayInfo,
            *mut *mut ImageOutput,
            usize,
        ) -> crate::base::Status};
        pub type GetFontInfoExSignature = eficall! {fn(
            *const Protocol,
            *mut crate::protocols::hii_font::Handle,
            *const DisplayInfo,
            *mut *mut DisplayInfo,
            crate::protocols::hii_font::String,
        ) -> crate::base::Status};
        pub type GetGlyphInfoSignature = eficall! {fn(
            *const Protocol,
            crate::base::Char16,
            *const DisplayInfo,
            *mut crate::hii::GlyphInfo,
        ) -> crate::base::Status};
    }

    pub mod hii_font {
        use crate::protocols::hii_font::{Handle, OutFlags, Protocol, RowInfo, String};

        pub type StringToImageSignature = eficall! {fn(
            *const Protocol,
            OutFlags,
            String,
            *const crate::protocols::hii_font_ex::DisplayInfo,
            *mut *mut crate::protocols::hii_font_ex::ImageOutput,
            usize,
            usize,
            *mut *mut RowInfo,
            *mut usize,
            *mut usize,
        ) -> crate::base::Status};
        pub type StringIdToImageSignature = eficall! {fn(
            *const Protocol,
            OutFlags,
            crate::hii::Handle,
            crate::hii::StringId,
            *const crate::base::Char8,
            *const crate::protocols::hii_font_ex::DisplayInfo,
            *mut *mut crate::protocols::hii_font_ex::ImageOutput,
            usize,
            usize,
            *mut *mut RowInfo,
            *mut usize,
            *mut usize,
        ) -> crate::base::Status};
        pub type GetGlyphSignature = eficall! {fn(
            *const Protocol,
            crate::base::Char16,
            *const crate::protocols::hii_font_ex::DisplayInfo,
            *mut *mut crate::protocols::hii_font_ex::ImageOutput,
            *mut usize,
        ) -> crate::base::Status};
        pub type GetFontInfoSignature = eficall! {fn(
            *const Protocol,
            *mut Handle,
            *const crate::protocols::hii_font_ex::DisplayInfo,
            *mut *mut crate::protocols::hii_font_ex::DisplayInfo,
            String,
        ) -> crate::base::Status};
    }

    pub mod hii_database {
        use crate::protocols::hii_database::{KeyboardLayout, Notify, NotifyType, Protocol};

        pub type NewPackageListSignature = eficall! {fn(
            *const Protocol,
            *const crate::hii::PackageListHeader,
            crate::base::Handle,
            *mut crate::hii::Handle,
        ) -> crate::base::Status};
        pub type RemovePackageListSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
        ) -> crate::base::Status};
        pub type UpdatePackageListSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *const crate::hii::PackageListHeader,
        ) -> crate::base::Status};
        pub type ListPackageListsSignature = eficall! {fn(
            *const Protocol,
            u8,
            *const crate::base::Guid,
            *mut usize,
            *mut crate::hii::Handle,
        ) -> crate::base::Status};
        pub type ExportPackageListsSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *mut usize,
            *mut crate::hii::PackageListHeader,
        ) -> crate::base::Status};
        pub type RegisterPackageNotifySignature = eficall! {fn(
            *const Protocol,
            u8,
            *const crate::base::Guid,
            Notify,
            NotifyType,
            *mut crate::base::Handle,
        ) -> crate::base::Status};
        pub type UnregisterPackageNotifySignature = eficall! {fn(
            *const Protocol,
            crate::base::Handle,
        ) -> crate::base::Status};
        pub type FindKeyboardLayoutsSignature = eficall! {fn(
            *const Protocol,
            *mut u16,
            *mut crate::base::Guid,
        ) -> crate::base::Status};
        pub type GetKeyboardLayoutSignature = eficall! {fn(
            *const Protocol,
            *const crate::base::Guid,
            *mut u16,
            *mut KeyboardLayout,
        ) -> crate::base::Status};
        pub type SetKeyboardLayoutSignature = eficall! {fn(
            *const Protocol,
            *mut crate::base::Guid,
        ) -> crate::base::Status};
        pub type GetPackageListHandleSignature = eficall! {fn(
            *const Protocol,
            crate::hii::Handle,
            *mut crate::base::Handle,
        ) -> crate::base::Status};

        pub type NotifySignature = eficall! {fn(
            u8,
            *const crate::base::Guid,
            *const crate::hii::PackageHeader,
            crate::hii::Handle,
            NotifyType,
        ) -> crate::base::Status};
    }

    pub mod graphics_output {
        use crate::protocols::graphics_output::{
            BltOperation, BltPixel, ModeInformation, Protocol,
        };

        pub type QueryModeSignature = eficall! {fn(
            *mut Protocol,
            u32,
            *mut usize,
            *mut *mut ModeInformation,
        ) -> crate::base::Status};
        pub type SetModeSignature = eficall! {fn(
            *mut Protocol,
            u32,
        ) -> crate::base::Status};
        pub type BltSignature = eficall! {fn(
            *mut Protocol,
            *mut BltPixel,
            BltOperation,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        ) -> crate::base::Status};
    }

    pub mod file {
        use crate::protocols::file::{IoToken, Protocol};

        pub type OpenSignature = eficall! {fn(
            *mut Protocol,
            *mut *mut Protocol,
            *mut crate::base::Char16,
            u64,
            u64,
        ) -> crate::base::Status};
        pub type CloseSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type DeleteSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type ReadSignature = eficall! {fn(
            *mut Protocol,
            *mut usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type WriteSignature = eficall! {fn(
            *mut Protocol,
            *mut usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type GetPositionSignature = eficall! {fn(
            *mut Protocol,
            *mut u64,
        ) -> crate::base::Status};
        pub type SetPositionSignature = eficall! {fn(
            *mut Protocol,
            u64,
        ) -> crate::base::Status};
        pub type GetInfoSignature = eficall! {fn(
            *mut Protocol,
            *mut crate::base::Guid,
            *mut usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type SetInfoSignature = eficall! {fn(
            *mut Protocol,
            *mut crate::base::Guid,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type FlushSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type OpenExSignature = eficall! {fn(
            *mut Protocol,
            *mut *mut Protocol,
            *mut crate::base::Char16,
            u64,
            u64,
            *mut IoToken,
        ) -> crate::base::Status};
        pub type ReadExSignature = eficall! {fn(
            *mut Protocol,
            *mut IoToken,
        ) -> crate::base::Status};
        pub type WriteExSignature = eficall! {fn(
            *mut Protocol,
            *mut IoToken,
        ) -> crate::base::Status};
        pub type FlushExSignature = eficall! {fn(
            *mut Protocol,
            *mut IoToken,
        ) -> crate::base::Status};
    }

    pub mod driver_binding {
        use crate::protocols::driver_binding::Protocol;

        pub type SupportedSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Handle,
            *mut crate::protocols::device_path::Protocol,
        ) -> crate::base::Status};
        pub type StartSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Handle,
            *mut crate::protocols::device_path::Protocol,
        ) -> crate::base::Status};
        pub type StopSignature = eficall! {fn(
            *mut Protocol,
            crate::base::Handle,
            usize,
            *mut crate::base::Handle,
        ) -> crate::base::Status};
    }

    pub mod disk_io2 {
        use crate::protocols::disk_io2::{Protocol, Token};

        pub type CancelSignature = eficall! {fn(
            *mut Protocol,
        ) -> crate::base::Status};
        pub type ReadDiskExSignature = eficall! {fn(
            *mut Protocol,
            u32,
            u64,
            *mut Token,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type WriteDiskExSignature = eficall! {fn(
            *mut Protocol,
            u32,
            u64,
            *mut Token,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type FlushDiskExSignature = eficall! {fn(
            *mut Protocol,
            *mut Token,
        ) -> crate::base::Status};
    }

    pub mod disk_io {
        use crate::protocols::disk_io::Protocol;

        pub type ReadDiskSignature = eficall! {fn(
            *mut Protocol,
            u32,
            u64,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
        pub type WriteDiskSignature = eficall! {fn(
            *mut Protocol,
            u32,
            u64,
            usize,
            *mut core::ffi::c_void,
        ) -> crate::base::Status};
    }

    pub mod device_path_utilities {
        pub type GetDevicePathSizeSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
        ) -> usize};
        pub type DuplicateDevicePathSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
        ) -> *mut crate::protocols::device_path::Protocol};
        pub type AppendDevicePathSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
            *const crate::protocols::device_path::Protocol,
        ) -> *mut crate::protocols::device_path::Protocol};
        pub type AppendDeviceNodeSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
            *const crate::protocols::device_path::Protocol,
        ) -> *mut crate::protocols::device_path::Protocol};
        pub type AppendDevicePathInstanceSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
            *const crate::protocols::device_path::Protocol,
        ) -> *mut crate::protocols::device_path::Protocol};
        pub type GetNextDevicePathInstanceSignature = eficall! {fn(
            *mut *mut crate::protocols::device_path::Protocol,
            *mut usize,
        ) -> *mut crate::protocols::device_path::Protocol};
        pub type IsDevicePathMultiInstanceSignature = eficall! {fn(
            *const crate::protocols::device_path::Protocol,
        ) -> crate::base::Boolean};
        pub type CreateDeviceNodeSignature = eficall! {fn(
            u8,
            u8,
            u16,
        ) -> *mut crate::protocols::device_path::Protocol};
    }

    pub mod decompress {
        use crate::protocols::decompress::Protocol;

        pub type GetInfoSignature = eficall! {fn(
            *mut Protocol,
            *mut core::ffi::c_void,
            u32,
            *mut u32,
            *mut u32,
        ) -> crate::base::Status};
        pub type DecompressSignature = eficall! {fn(
            *mut Protocol,
            *mut core::ffi::c_void,
            u32,
            *mut core::ffi::c_void,
            u32,
            *mut core::ffi::c_void,
            u32,
        ) -> crate::base::Status};
    }
}

pub mod vendor {
    pub mod intel {
        pub mod console_control {
            use crate::vendor::intel::console_control::{Protocol, ScreenMode};

            pub type GetModeSignature = eficall! {fn(
                *mut Protocol,
                *mut ScreenMode,
                *mut crate::base::Boolean,
                *mut crate::base::Boolean,
            ) -> crate::base::Status};
            pub type SetModeSignature = eficall! {fn(
                *mut Protocol,
                ScreenMode,
            ) -> crate::base::Status};
            pub type LockStdInSignature = eficall! {fn(
                *mut Protocol,
                *mut crate::base::Char16,
            ) -> crate::base::Status};
        }
    }
}
