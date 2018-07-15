/// Copied from wasm32-unknown-unknown

use super::{LinkerFlavor, Target, TargetOptions, PanicStrategy};

pub fn target() -> Result<Target, String> {
    let opts = TargetOptions {
        cpu: "sm_50".to_string(),
        linker: Some("llvm-link".into()),
        dynamic_linking: true,
        only_cdylib: true,
        executables: false,
        exe_suffix: ".bc".to_string(),
        dll_suffix: ".bc".to_string(),
        singlethread: true,
        obj_is_bitcode: true,
        panic_strategy: PanicStrategy::Abort,
        .. Default::default()
    };
    Ok(Target {
        llvm_target: "nvptx64-nvidia-cuda".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        target_env: "".to_string(),
        target_os: "cuda".to_string(),
        target_vendor: "nvidia".to_string(),
        data_layout: "e-i64:64-i128:128-v16:16-v32:32-n16:32:64".to_string(),
        arch: "nvptx64".to_string(),
        linker_flavor: LinkerFlavor::LlvmLink,
        options: opts,
    })
}

