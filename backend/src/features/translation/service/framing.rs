const UNIX_FRAME_SEPARATOR: &[u8] = b"\n\n";
const WINDOWS_FRAME_SEPARATOR: &[u8] = b"\r\n\r\n";

/// Finds the next complete server-sent-event frame in a byte buffer.
pub(super) fn sse_frame_end(buffer: &[u8]) -> Option<(usize, usize)> {
    buffer
        .windows(WINDOWS_FRAME_SEPARATOR.len())
        .position(|window| window == WINDOWS_FRAME_SEPARATOR)
        .map(|index| (index, WINDOWS_FRAME_SEPARATOR.len()))
        .or_else(|| {
            buffer
                .windows(UNIX_FRAME_SEPARATOR.len())
                .position(|window| window == UNIX_FRAME_SEPARATOR)
                .map(|index| (index, UNIX_FRAME_SEPARATOR.len()))
        })
}

#[cfg(test)]
#[path = "framing_test.rs"]
mod framing_test;
