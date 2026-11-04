use super::*;
#[test]
fn control_requires_bounded_exact_frames_and_preserves_distinct_messages() {
    let mut input = &b"{\"phase\":\"prepare\",\"index\":0}\n{\"phase\":\"map\"}\n"[..];
    assert!(matches!(
        read_frame(&mut input),
        Ok(Frame::Prepare {
            index: 0,
            source_run: None
        })
    ));
    assert!(matches!(read_frame(&mut input), Ok(Frame::Map {})));
    assert!(matches!(read_frame(&mut input), Err(Failure::Transport)));
    for bytes in [
        b"{\"phase\":\"map\",\"retry\":true}\n".as_slice(),
        b"{\"phase\":\"unknown\"}\n",
        b"[]\n",
    ] {
        assert!(matches!(read_frame(&mut &*bytes), Err(Failure::Arguments)));
    }
    assert!(matches!(
        read_frame(&mut &b"{\"phase\":\"map\"}"[..]),
        Err(Failure::ReplyLimit)
    ));
    assert!(matches!(
        read_frame(&mut vec![b'x'; 8193].as_slice()),
        Err(Failure::ReplyLimit)
    ));
}
