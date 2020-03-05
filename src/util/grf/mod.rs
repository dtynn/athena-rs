use std::path::PathBuf;

enum FileListType {
    File = 0x01,
    EncryptHeader = 0x04,
    EncryptMixed = 0x02,
}

struct FileList {
    src_len: i32,
    src_len_aligned: i32,
    dec_len: i32,
    scr_pos: i32,
    next: i32,
    typ: u8,
    fname: [u8; 128 - 4 * 5],
    fnd: Vec<u8>,
    g_entry: u8,
}

struct GRF {
    dir: PathBuf,
    file_list: FileList,
    entries: usize,
    max_entries: usize,

    g_entry_table: Vec<Vec<u8>>,
    g_entry_entries: usize,
    g_entry_max_entries: usize,
}

fn nibble_swap(src: &mut [u8]) {
    for c in src {
        let origin = *c;
        *c = origin << 4 | origin >> 4;
    }
}

fn substitution(src: u8) -> u8 {
    match src {
        0x00 => 0x2B,
        0x2B => 0x00,
        0x6C => 0x80,
        0x01 => 0x68,
        0x68 => 0x01,
        0x48 => 0x77,
        0x60 => 0xFF,
        0x77 => 0x48,
        0xB9 => 0xC0,
        0xC0 => 0xB9,
        0xFE => 0xEB,
        0xEB => 0xFE,
        0x80 => 0x6C,
        0xFF => 0x60,
        _ => src,
    }
}

fn shuffle_decode(src: &mut [u8]) {
    let mut out = [0u8; 8];

    out[0] = src[3];
    out[1] = src[4];
    out[2] = src[6];
    out[3] = src[0];
    out[4] = src[1];
    out[5] = src[2];
    out[6] = src[5];
    out[7] = substitution(src[7]);

    src.copy_from_slice(&out[..]);
}

fn decode_header() {
    unimplemented!()
}
