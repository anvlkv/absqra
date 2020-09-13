macro_rules! repeat_kindly {
    ($find: ident, $blk: block, $create_type:ident) => {
        if $create_type::starts_with_tokens()
            .into_iter()
            .find($find)
            .is_some() {
                $blk
            }
    };
    ($find: ident, $blk: block, $x: ident, $($y:ident), +) => {
        repeat_kindly!($find, $blk, $x);
        repeat_kindly!($find, $blk, $($y),+);
    };
}