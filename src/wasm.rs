// Copyright 2021 Miguel Peláez <kernelfreeze@outlook.com>
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

use anyhow::{Error, Result};
use bare_io::{Read, Write};
use wain_exec::Runtime;

use self::importer::DefaultImporter;
use crate::prelude::*;

mod importer;

pub async fn load_binary_program<R: Read, W: Write>(source: &[u8], stdin: R, stdout: W) -> Result<()> {
    let tree = wain_syntax_binary::parse(&source)
        .map_err(|e| format!("Failed to parse binary. Error: {:?}", e))
        .map_err(Error::msg)?;

    wain_validate::validate(&tree)
        .map_err(|e| format!("Failed to validate binary. Error: {:?}", e))
        .map_err(Error::msg)?;

    let importer = DefaultImporter::with_stdio(stdin, stdout);

    if let Ok(mut runtime) = Runtime::instantiate(&tree.module, importer) {
        if runtime.module().entrypoint.is_none() {
            runtime.invoke("_start", &[]).map_err(Error::msg)?;
        }
    }

    Ok(())
}

pub async fn load_text_program<R: Read, W: Write>(source: &str, stdin: R, stdout: W) -> Result<()> {
    let tree = wain_syntax_text::parse(&source)
        .map_err(|e| format!("Failed to parse binary. Error: {:?}", e))
        .map_err(Error::msg)?;

    wain_validate::validate(&tree)
        .map_err(|e| format!("Failed to validate binary. Error: {:?}", e))
        .map_err(Error::msg)?;

    let importer = DefaultImporter::with_stdio(stdin, stdout);

    if let Ok(mut runtime) = Runtime::instantiate(&tree.module, importer) {
        if runtime.module().entrypoint.is_none() {
            runtime.invoke("_start", &[]).map_err(Error::msg)?;
        }
    }

    Ok(())
}
