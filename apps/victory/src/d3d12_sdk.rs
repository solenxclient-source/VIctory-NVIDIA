//! Opt-in to the D3D12 Agility SDK.

#[used]
#[no_mangle]
pub static D3D12SDKVersion: u32 = 620;

#[used]
#[no_mangle]
pub static D3D12SDKPath: [u8; 9] = *b".\\D3D12\\\0";
