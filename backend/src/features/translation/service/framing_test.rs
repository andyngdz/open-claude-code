use super::sse_frame_end;

#[test]
fn finds_unix_and_windows_event_boundaries() {
    assert_eq!(sse_frame_end(b"data: {}\n\n"), Some((8, 2)));
    assert_eq!(sse_frame_end(b"data: {}\r\n\r\n"), Some((8, 4)));
}
