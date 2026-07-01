#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding { Utf8, Utf8Bom, Utf16Le, Utf16Be, Unknown }

pub fn detect_encoding(bytes: &[u8]) -> Encoding {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { return Encoding::Utf8Bom; }
    if bytes.starts_with(&[0xFF, 0xFE]) { return Encoding::Utf16Le; }
    if bytes.starts_with(&[0xFE, 0xFF]) { return Encoding::Utf16Be; }
    if std::str::from_utf8(bytes).is_ok() { return Encoding::Utf8; }
    Encoding::Unknown
}
