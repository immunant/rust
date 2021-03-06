// This is a test of the borrowck migrate mode. It leverages #38899, a
// bug that is fixed by NLL: this code is (unsoundly) accepted by
// AST-borrowck, but is correctly rejected by the NLL borrowck.
//
// Therefore, for backwards-compatiblity, under borrowck=migrate the
// NLL checks will be emitted as *warnings*.
//
// In Rust 2018, no errors will be downgraded to warnings.

// NLL mode makes this compile-fail; we cannot currently encode a
// test that is run-pass or compile-fail based on compare-mode. So
// just ignore it instead:

// ignore-compare-mode-nll
// ignore-compare-mode-polonius

// revisions: zflag edition
//[zflag]compile-flags: -Z borrowck=migrate
//[edition]edition:2018
//[zflag] check-pass

pub struct Block<'a> {
    current: &'a u8,
    unrelated: &'a u8,
}

fn bump<'a>(mut block: &mut Block<'a>) {
    let x = &mut block;
    let p: &'a u8 = &*block.current;
    //[edition]~^ ERROR cannot borrow `*block.current` as immutable
    // (use `x` and `p` so enabling NLL doesn't assign overly short lifetimes)
    drop(x);
    drop(p);
}

fn main() {}
