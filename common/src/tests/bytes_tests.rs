use bytes::{BytesMut, BufMut};

#[test]
fn buf_works() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);

    let a = buf.split();
    assert_eq!(a, b"hello world\x04\xD2"[..]);

    buf.put(&b"goodbye world"[..]);

    let b = buf.split();
    assert_eq!(b, b"goodbye world"[..]);

    assert_eq!(buf.capacity(), 998);
}

#[test]
fn enumerate_works() {
    let bytes=b"hello world";
    let mut buf = BytesMut::with_capacity(1024);
    println!("buf len {}", buf.capacity());
    let mut write_count=0;
    for (i, b) in bytes.iter().enumerate() {
        buf.put_u32(i as u32);
        buf.put_u8( *b);
        write_count += 1;
    }
    println!("write count {}: {:?}", write_count, buf.to_vec());
}

