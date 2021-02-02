// MIT License
//
// Copyright (c) 2021 The etheryal Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod modules;

use log::info;
use wasmi::{ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue};

use self::modules::wasi::WasiImportResolver;

/// Run a Webassembly program
// source: &[u8]
pub async fn run_binary_program() {
    let source = include_bytes!("test.wasm");
    let module = Module::from_buffer(source).unwrap();
    let mut import_resolver = ImportsBuilder::default();

    let wasi_resolver = WasiImportResolver::new();
    import_resolver.push_resolver("wasi_snapshot_preview1", &wasi_resolver);

    let main = ModuleInstance::new(&module, &import_resolver)
        .expect("Failed to instantiate module")
        .async_run_start(&mut NopExternals, 10)
        .await
        .expect("Failed to run start function in module");

    info!(
        "Result: {:?}",
        main.async_invoke_export("_call", &[RuntimeValue::I32(0i32)], &mut NopExternals, 10)
            .await
    );
}
