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

/// Configuration for a sample HTTP filter.
pub struct SampleHttpFilterConfig {
    pub value: String,
}

impl SampleHttpFilterConfig {
    /// Creates a new configuration.
    pub fn new<T: Into<String>>(value: T) -> SampleHttpFilterConfig {
        SampleHttpFilterConfig {
            value: value.into(),
        }
    }
}

impl Default for SampleHttpFilterConfig {
    /// Creates the default configuration.
    fn default() -> Self {
        SampleHttpFilterConfig {
            value: String::new(),
        }
    }
}
