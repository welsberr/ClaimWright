use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Utf8Text,
    Pdf,
    Office,
    Image,
    Archive,
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInfo {
    pub kind: ArtifactKind,
    pub sha256: String,
    pub text_sha256: Option<String>,
}

pub fn inspect(path: &Path) -> Result<ArtifactInfo, String> {
    let bytes = fs::read(path).map_err(|e| format!("cannot read artifact: {e}"))?;
    let kind = classify(path, &bytes);
    let text_sha256 = if matches!(kind, ArtifactKind::Utf8Text) {
        Some(super::hash::sha256_hex(&bytes))
    } else {
        None
    };
    Ok(ArtifactInfo {
        kind,
        sha256: super::hash::sha256_hex(&bytes),
        text_sha256,
    })
}

fn classify(path: &Path, bytes: &[u8]) -> ArtifactKind {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if bytes.starts_with(b"%PDF-") || ext == "pdf" {
        return ArtifactKind::Pdf;
    }
    if ["doc", "docx", "odt", "rtf", "xls", "xlsx", "ppt", "pptx"].contains(&ext.as_str()) {
        return ArtifactKind::Office;
    }
    if ["png", "jpg", "jpeg", "gif", "webp", "tif", "tiff"].contains(&ext.as_str()) {
        return ArtifactKind::Image;
    }
    if ["zip", "tar", "gz", "bz2", "7z", "rar"].contains(&ext.as_str()) {
        return ArtifactKind::Archive;
    }
    if std::str::from_utf8(bytes).is_ok() {
        ArtifactKind::Utf8Text
    } else {
        ArtifactKind::Binary
    }
}

pub fn kind_name(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Utf8Text => "utf8_text",
        ArtifactKind::Pdf => "pdf",
        ArtifactKind::Office => "office",
        ArtifactKind::Image => "image",
        ArtifactKind::Archive => "archive",
        ArtifactKind::Binary => "binary",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn text_gets_text_hash() {
        let p = std::env::temp_dir().join("claimwright-artifact.txt");
        fs::write(&p, b"abc").unwrap();
        let i = inspect(&p).unwrap();
        assert_eq!(i.kind, ArtifactKind::Utf8Text);
        assert!(i.text_sha256.is_some());
        let _ = fs::remove_file(p);
    }
}
