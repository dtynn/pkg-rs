use super::{ZagZig, ZigZag};
use rand::{thread_rng, Rng};

#[test]
fn zigzag_test() {
    const REPEAT: usize = 10000;

    let mut rng = thread_rng();

    macro_rules! gen_zigzag_test {
        ($typ:ty) => {{
            let mut cases: Vec<$typ> = vec![0; REPEAT];
            rng.fill(&mut cases[..]);

            for c in cases {
                let got = c.zigzag().zagzig();
                assert_eq!(
                    c,
                    got,
                    "TYPE: {}, expected: {}, got: {}",
                    stringify!($typ),
                    c,
                    got
                );
            }
        }};
    }

    gen_zigzag_test!(i8);
    gen_zigzag_test!(i16);
    gen_zigzag_test!(i32);
    gen_zigzag_test!(i64);
    gen_zigzag_test!(i128);
}
