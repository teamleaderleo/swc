use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Writes a serialized module without exposing a partially-written cache file.
///
/// Parallel runners can compile the same plugin from multiple processes. A
/// unique temporary file in the cache directory prevents an interrupted or
/// concurrent writer from blocking future cache publication while keeping the
/// final rename atomic.
pub(crate) fn write_atomic(path: &Path, data: &[u8]) -> io::Result<()> {
    let (temporary_path, mut file) = create_temporary_file(path)?;

    if let Err(err) = file.write_all(data) {
        drop(file);
        let _ = std::fs::remove_file(&temporary_path);
        return Err(err);
    }
    drop(file);

    match std::fs::rename(&temporary_path, path) {
        Ok(()) => Ok(()),
        // Windows does not replace an existing file with rename. Another process
        // publishing the same content first is a successful cache write for us.
        Err(_) if path.is_file() => {
            let _ = std::fs::remove_file(&temporary_path);
            Ok(())
        }
        Err(err) => {
            let _ = std::fs::remove_file(&temporary_path);
            Err(err)
        }
    }
}

fn create_temporary_file(path: &Path) -> io::Result<(PathBuf, File)> {
    loop {
        let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut temporary_path = path.as_os_str().to_os_string();
        temporary_path.push(format!(".{}.{}.tmp", std::process::id(), id));
        let temporary_path = PathBuf::from(temporary_path);

        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temporary_path)
        {
            Ok(file) => return Ok((temporary_path, file)),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err),
        }
    }
}
