// Copyright 2020 Tetrate
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::extension::Result;
use crate::host;

use proxy_wasm::hostcalls;
use proxy_wasm::types::Bytes;

pub trait Lifecycle {
    fn on_start(&mut self, _vm_configuration_size: usize, _ops: &dyn StartOps) -> Result<bool> {
        Ok(true)
    }
}

pub trait StartOps {
    fn get_configuration(&self) -> host::Result<Option<Bytes>>;
}

pub struct Host;

impl StartOps for Host {
    fn get_configuration(&self) -> host::Result<Option<Bytes>> {
        hostcalls::get_configuration().map_err(|status| ("proxy_get_configuration", status))
    }
}
