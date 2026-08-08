use std::time::{SystemTime, UNIX_EPOCH};

use swc_plugin_backend_wasmtime::WasmtimeRuntime;
use swc_plugin_runner::runtime::Runtime;

#[test]
fn rejected_cache_file_is_removed() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "swc-wasmtime-rejected-cache-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();

    let cache_path = directory.join("plugin.wasmtime-v35");
    std::fs::write(&cache_path, b"not a serialized wasmtime module").unwrap();

    let runtime = WasmtimeRuntime;
    let loaded = unsafe { runtime.load_cache(&cache_path) };

    assert!(loaded.is_none(), "corrupt cache data must be rejected");
    assert!(
        !cache_path.exists(),
        "a rejected cache artifact must be removed so a fresh compiled cache can be published"
    );

    let _ = std::fs::remove_dir_all(&directory);
}
