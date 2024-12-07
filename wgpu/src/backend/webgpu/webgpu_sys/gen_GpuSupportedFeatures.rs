// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// These bindings are vendored into wgpu for the sole purpose of letting
// us pin the WebGPU backend to a specific version of the bindings, not
// to enable local changes. There are no provisions to preserve changes
// you make here the next time we re-vendor the bindings.
//
// The `web-sys` crate does not treat breaking changes to the WebGPU API
// as semver breaking changes, as WebGPU is "unstable". This means Cargo
// will not let us mix versions of `web-sys`, pinning WebGPU bindings to
// a specific version, while letting other bindings like WebGL get
// updated. Vendoring WebGPU was the workaround we chose.
//
// Vendoring also allows us to avoid building `web-sys` with
// `--cfg=web_sys_unstable_apis`, needed to get the WebGPU bindings.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version 0.2.97` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUSupportedFeatures , typescript_type = "GPUSupportedFeatures")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSupportedFeatures` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSupportedFeatures;

    # [wasm_bindgen (structural , method , getter , js_class = "GPUSupportedFeatures" , js_name = size)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &GpuSupportedFeatures) -> u32;

    # [wasm_bindgen (method , structural , js_class = "GPUSupportedFeatures" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(this: &GpuSupportedFeatures) -> ::js_sys::Iterator;

    # [wasm_bindgen (catch , method , structural , js_class = "GPUSupportedFeatures" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &GpuSupportedFeatures,
        callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;

    # [wasm_bindgen (method , structural , js_class = "GPUSupportedFeatures" , js_name = has)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &GpuSupportedFeatures, value: &str) -> bool;

    # [wasm_bindgen (method , structural , js_class = "GPUSupportedFeatures" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &GpuSupportedFeatures) -> ::js_sys::Iterator;

    # [wasm_bindgen (method , structural , js_class = "GPUSupportedFeatures" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &GpuSupportedFeatures) -> ::js_sys::Iterator;
}
