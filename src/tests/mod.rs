mod idx {
    use crate::{
        Pack, Tid, cfg,
        page::{self, slot},
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn tid_roundtrips(tid in 0usize..Tid::<cfg::DefaultConfig>::BITS) {
            let tid = Tid::<cfg::DefaultConfig>::from_usize(tid);
            let packed = tid.pack(0);
            assert_eq!(tid, Tid::from_packed(packed));
        }

        #[test]
        fn idx_roundtrips(
            tid in 0usize..Tid::<cfg::DefaultConfig>::BITS,
            r#gen in 0usize..slot::Generation::<cfg::DefaultConfig>::BITS,
            addr in 0usize..page::Addr::<cfg::DefaultConfig>::BITS,
        ) {
            let tid = Tid::<cfg::DefaultConfig>::from_usize(tid);
            let r#gen = slot::Generation::<cfg::DefaultConfig>::from_usize(r#gen);
            let addr = page::Addr::<cfg::DefaultConfig>::from_usize(addr);
            let packed = tid.pack(r#gen.pack(addr.pack(0)));
            assert_eq!(addr, page::Addr::from_packed(packed));
            assert_eq!(r#gen, slot::Generation::from_packed(packed));
            assert_eq!(tid, Tid::from_packed(packed));
        }
    }
}

mod custom_config;

mod properties;
