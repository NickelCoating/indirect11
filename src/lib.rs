mod dxgi;

use com::runtime::{init_runtime, create_instance};

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

        Ok(Indirect11 {})
    }
}