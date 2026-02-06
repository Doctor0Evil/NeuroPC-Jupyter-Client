use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Minimal ALN shard loader: currently treats ALN as UTF-8 text.
///
/// In a full implementation, this would parse ALN syntax into structured types.
pub fn load_aln_shard(path: impl AsRef<Path>) -> std::io::Result<String> {
    let mut f = fs::File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn save_aln_shard(path: impl AsRef<Path>, data: &str) -> std::io::Result<()> {
    let mut f = fs::File::create(path)?;
    f.write_all(data.as_bytes())
}
