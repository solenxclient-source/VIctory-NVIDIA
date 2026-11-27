//! Opt-in to the D3D12 Agility SDK.
//!
//! Without these two exports `d3d12.dll` loads the system `D3D12Core.dll`,
//! which on Windows 10 tops out at feature level 12_1: no mesh shaders, no
//! Shader Model 6.6, so the layer cannot create its device. With them it loads
//! `D3D12\\D3D12Core.dll` from beside the program instead.
//!
//! The shape matters. Microsoft declares the pair as
//!
//! ```c
//! extern "C" __declspec(dllexport) extern const UINT  D3D12SDKVersion;
//! extern "C" __declspec(dllexport) extern const char* D3D12SDKPath;
//! ```
//!
//! `D3D12SDKPath` is a *pointer*, not a character array. Exporting the bytes
//! themselves makes `d3d12.dll` read the first eight characters of the path as
//! an address and fault, which surfaces as a silent 0xC0000142 at startup.
//!
//! `D3D12SDKVersion` must equal the Agility SDK release shipped in that folder
//! (1.620.x -> 620).
//!
//! `no_mangle` is an unsafe attribute since Rust 1.82, hence the wrapper.

/// `*const u8` is not `Sync`, and a `static` must be. The pointer is never
/// written after linking, so sharing it is sound.
#[repr(transparent)]
struct SyncPtr(*const u8);
unsafe impl Sync for SyncPtr {}

static SDK_PATH: [u8; 9] = *b".\\D3D12\\\0";

#[used]
#[unsafe(no_mangle)]
pub static D3D12SDKVersion: u32 = 620;

#[used]
#[unsafe(no_mangle)]
pub static D3D12SDKPath: SyncPtr = SyncPtr(&SDK_PATH as *const [u8; 9] as *const u8);
