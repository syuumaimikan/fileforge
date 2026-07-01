use fileforge_core::Result;
use crate::Encoding;

pub fn decode_bytes(encoding: Encoding, bytes: &[u8]) -> Result<String> {
    match encoding {
        Encoding::Utf8 => Ok(String::from_utf8(bytes.to_vec())?),
        Encoding::Utf8Bom => {
            let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
            Ok(String::from_utf8(bytes.to_vec())?)
        }
        Encoding::Utf16Le => { let bytes = bytes.strip_prefix(&[0xFF, 0xFE]).unwrap_or(bytes); decode_utf16(bytes, true) }
        Encoding::Utf16Be => { let bytes = bytes.strip_prefix(&[0xFE, 0xFF]).unwrap_or(bytes); decode_utf16(bytes, false) }
        Encoding::Unknown => anyhow::bail!("unknown encoding"),
    }
}

fn decode_utf16(bytes: &[u8], little_endian: bool) -> Result<String> {
    let mut u16s = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let value = if little_endian { u16::from_le_bytes([chunk[0], chunk[1]]) } else { u16::from_be_bytes([chunk[0], chunk[1]]) };
        u16s.push(value);
    }
    Ok(String::from_utf16(&u16s)?)
}
