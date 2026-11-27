//! Opt-in to the D3D12 Agility SDK.
//!
//! Without these two exports `d3d12.dll` loads the system `D3D12Core.dll`,
//! which on Windows 10 tops out at feature level 12_1: no mesh shaders, no
//! Shader Model 6.6, so the layer cannot create its device. With them it loads
//! `D3D12\\D3D12Core.dll` from beside the program instead.
//!
//! `D3D12SDKVersion` must equal the Agility SDK release shipped in that folder
//! (1.620.x -> 620). `D3D12SDKPath` is relative to the executable, UTF-8,
//! NUL-terminated, at most 256 bytes.
//!
//! `no_mangle` is an unsafe attribute since Rust 1.82, hence the wrapper.

#[used]
#[unsafe(no_mangle)]
pub static D3D12SDKVersion: u32 = 620;

#[used]
#[unsafe(no_mangle)]
pub static D3D12SDKPath: [u8; 9] = *b".\\D3D12\\\0";
