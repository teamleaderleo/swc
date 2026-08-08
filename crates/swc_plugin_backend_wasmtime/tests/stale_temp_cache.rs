use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use swc_plugin_backend_wasmtime::WasmtimeRuntime;
use swc_plugin_runner::runtime::Runtime;

fn legacy_temporary_path(path: &Path) -> PathBuf {
    let mut ext = path.extension().unwrap_or_default().to_owned();
    ext.push(".tmp");
    path.with_extension(ext)
}

#[test]
fn stale_legacy_temp_file_does_not_block_cache_publication() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "swc-wasmtime-stale-temp-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();

    let cache_path = directory.join("plugin.wasmtime-v35");
    let stale_temp_path = legacy_temporary_path(&cache_path);
    std::fs::write(&stale_temp_path, b"interrupted partial cache").unwrap();
    assert!(stale_temp_path.is_file());

    let runtime = WasmtimeRuntime;
    let cache = runtime
        .prepare_module(b"\0asm\x01\0\0\0")
        .expect("empty wasm module should compile");
    runtime
        .store_cache(&cache_path, &cache)
        .expect("cache store reports success");

    let cache_was_published = cache_path.is_file();
    let _ = std::fs::remove_dir_all(&directory);

    assert!(
        cache_was_published,
        "a stale legacy .tmp file must not turn a successful store into a missing cache entry"
    );
}
