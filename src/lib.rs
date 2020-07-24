// Points to remember!
//
// * Com interface GUIDs are in hex format of 8-4-4-4-12.
// * The terminology "COM interface" is not the same as "Rust trait" or "C# interface".
// * Most interfaces derive from IUnknown directly or indirectly.
// * Client side COM is the code that consumes COM based APIs. Server side COM creates COM based API. Using DirectX is client side.
// * Client side COM is much simpler than Server side COM.
// * COM ain't safe by default. But it can mimic Rust grade safe behavior with programmer effort.
// * Methods/fields can have any name, but they most match the order and number on the VTable. The signature/type must match, too.

use com::runtime::{init_runtime, create_instance};
use com::interfaces::iunknown::IUnknown;
use com::com_interface;
use com::sys::{HRESULT, NOERROR};

#[com_interface("7B7166EC-21C7-44AE-B21A-C9AE321AE369")]
pub trait IID_IDXGIFactory: IUnknown
{
    //unsafe fn method_name(&self) -> HRESULT;
}

/// Holds basic info and instances.
pub struct Indirect11
{
    //device:
}

impl Indirect11
{
    /// Will create instance for all used com interfaces that are supported.
    pub fn new() -> Result<Indirect11, ()>
    {
        init_runtime().unwrap();

        println!("Print this only for testing cargo and git dependencies now.");

        Ok(Indirect11 {})
    }
}