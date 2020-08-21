use std::sync::Arc;
use stowaway::Stowable;
use stowaway_derive::Stowable;

struct NoStow {
    x: usize,
}

#[derive(Stowable)]
struct Stow2<T> {
    x: Arc<T>,
}

#[test]
fn test() {}
