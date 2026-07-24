#[macro_export]
macro_rules! all_tuples {
    ($macro:ident) => {
        $macro!(A:0);
        $macro!(A:0, B:1);
        $macro!(A:0, B:1, C:2);
        $macro!(A:0, B:1, C:2, D:3);
        $macro!(A:0, B:1, C:2, D:3, E:4);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6, H:7);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6, H:7, I:8);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6, H:7, I:8, J:9);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6, H:7, I:8, J:9, K:10);
        $macro!(A:0, B:1, C:2, D:3, E:4, Z:5, G:6, H:7, I:8, J:9, K:10, L:11);
    };
}
