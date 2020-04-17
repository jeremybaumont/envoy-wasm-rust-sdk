extern crate std;
use std::prelude::v1::*;

pub mod ops;
pub mod context;
pub use context::FactoryContext; 

use crate::envoy::host;
use crate::envoy::extension::Result;

use proxy_wasm::types::Bytes;

pub trait Factory {
    type Extension;

    fn on_configure(&mut self, _plugin_configuration_size: usize, _ops: &dyn ConfigureOps) -> Result<bool> {
        Ok(true)
    }

    fn new_extension(&mut self, _context_id: u32) -> Result<Box<Self::Extension>>;

    fn on_drain(&mut self, _ops: &dyn DrainOps) -> Result<bool> {
        Ok(true)
    }
}

pub trait ConfigureOps {
    fn get_configuration(&self) -> host::Result<Option<Bytes>>;
}

pub trait DrainOps {
    fn done(&self) -> host::Result<()>;
}

pub trait Ops: ConfigureOps + DrainOps where Self: std::marker::Sized {}

impl<T> Ops for T where T: ConfigureOps + DrainOps {}
