use blake3;

const MIN_CHUNK: usize = 16 * 1024;
const MAX_CHUNK: usize = 256 * 1024;

/// Content-defined chunking using simple rolling pattern
pub fn get_chunks(data: &[u8]) -> Vec<(usize, usize)> {
    let mut chunks = Vec::new();
    let mut last_cut = 0;

    if data.len() <= MAX_CHUNK {
        chunks.push((0, data.len()));
        return chunks;
    }

    let mut i = MIN_CHUNK;
    while i < data.len() {
        let chunk_size = i - last_cut;
        let hash_pattern = 0x1FFF;

        if (i < data.len()
            && ((data[i] as u32 | ((data[i - 1] as u32) << 8)) & hash_pattern == 0))
            || chunk_size >= MAX_CHUNK
        {
            if chunk_size >= MIN_CHUNK {
                chunks.push((last_cut, chunk_size));
                last_cut = i;
                i += MIN_CHUNK;
                continue;
            }
        }
        i += 1;
    }

    if last_cut < data.len() {
        chunks.push((last_cut, data.len() - last_cut));
    }

    chunks
}
