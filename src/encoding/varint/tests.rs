use super::{get_i64, get_u64, put_i64, put_u64, ErrorKind, MAX_VARINT_LEN_U64};
use rand::{thread_rng, Rng};

#[test]
fn uvarint_test() {
    let mut rng = thread_rng();
    let mut buf = Vec::with_capacity(MAX_VARINT_LEN_U64);

    let mut nums = vec![0; 10000];
    rng.fill(&mut nums[..]);
    for num in nums {
        buf.clear();

        let written = put_u64(&mut buf, num);
        assert!(
            written <= MAX_VARINT_LEN_U64,
            "written bytes should be less or equal to {}, got {}, number={}",
            MAX_VARINT_LEN_U64,
            written,
            num
        );

        assert_eq!(
            get_u64(&buf[..written - 1]),
            Err(ErrorKind::ShortOfData(written - 1)),
            "data w/o last byte should lead to a short of data error"
        );

        let decoded = get_u64(&buf[..]);
        assert_eq!(
            decoded,
            Ok((num, written)),
            "expected decoded u64 number {}, bytes length {},  got {:?}, buffer content={:?}",
            num,
            written,
            decoded,
            buf
        );
    }
}

#[test]
fn varint_test() {
    let mut rng = thread_rng();
    let mut buf = Vec::with_capacity(MAX_VARINT_LEN_U64);

    let mut nums = vec![0; 10000];
    rng.fill(&mut nums[..]);
    for num in nums {
        buf.clear();

        let written = put_i64(&mut buf, num);
        assert!(
            written <= MAX_VARINT_LEN_U64,
            "written bytes should be less or equal to {}, got {}, number={}",
            MAX_VARINT_LEN_U64,
            written,
            num
        );

        assert_eq!(
            get_i64(&buf[..written - 1]),
            Err(ErrorKind::ShortOfData(written - 1)),
            "data w/o last byte should lead to a short of data error"
        );

        let decoded = get_i64(&buf[..]);
        assert_eq!(
            decoded,
            Ok((num, written)),
            "expected decoded i64 number {}, bytes length {}, got {:?}, buffer content={:?}",
            num,
            written,
            decoded,
            buf
        );
    }
}

#[test]
fn uvarint_error_test() {
    let data = vec![
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x79,
    ];

    assert!(
        get_u64(&data[5..]).is_ok(),
        "shoud be valid data of length 7"
    );

    assert!(
        get_u64(&data[2..]).is_ok(),
        "shoud be valid data of length 10"
    );

    assert_eq!(
        get_u64(&data[..9]),
        Err(ErrorKind::ShortOfData(9)),
        "expected err short of data for &data[..9]"
    );

    assert_eq!(
        get_u64(&data[..10]),
        Err(ErrorKind::Overflow(10)),
        "expected err overflow for &data[..10]"
    );

    assert_eq!(
        get_u64(&data[..11]),
        Err(ErrorKind::Overflow(10)),
        "expected err overflow for &data[..11]"
    );
}
