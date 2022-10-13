use narrate::{Chain, Error};

use crate::util::{ErrorStub, TestError};

#[test]
fn from_nested_error() {
    let error = Error::new(TestError::Stub(ErrorStub));
    let mut chain = error.chain();
    assert_eq!(
        TestError::Stub(ErrorStub).to_string(),
        next_string(&mut chain)
    );
    assert_eq!(ErrorStub.to_string(), next_string(&mut chain));
    assert!(chain.next().is_none());
}

#[test]
fn wrapped_nested_error() {
    let context = "context";
    let error = Error::new(TestError::Stub(ErrorStub)).wrap(context);
    let mut chain = error.chain();
    assert_eq!(context, next_string(&mut chain));
    assert_eq!(
        TestError::Stub(ErrorStub).to_string(),
        next_string(&mut chain)
    );
    assert_eq!(ErrorStub.to_string(), next_string(&mut chain));
    assert!(chain.next().is_none());
}

fn next_string(chain: &mut Chain) -> String {
    chain.next().expect("error source").to_string()
}

// Rest of tests copied from anyhow
// https://github.com/dtolnay/anyhow/blob/1.0.65/tests/test_chain.rs

#[test]
fn iter() {
    let e = error();
    let mut chain = e.chain();
    assert_eq!("3", chain.next().unwrap().to_string());
    assert_eq!("2", chain.next().unwrap().to_string());
    assert_eq!("1", chain.next().unwrap().to_string());
    assert_eq!("0", chain.next().unwrap().to_string());
    assert!(chain.next().is_none());
    assert!(chain.next_back().is_none());
}

#[test]
fn iter_rev() {
    let e = error();
    let mut chain = e.chain().rev();
    assert_eq!("0", chain.next().unwrap().to_string());
    assert_eq!("1", chain.next().unwrap().to_string());
    assert_eq!("2", chain.next().unwrap().to_string());
    assert_eq!("3", chain.next().unwrap().to_string());
    assert!(chain.next().is_none());
    assert!(chain.next_back().is_none());
}

#[test]
fn len() {
    let e = error();
    let mut chain = e.chain();
    assert_eq!(4, chain.len());
    assert_eq!((4, Some(4)), chain.size_hint());
    assert_eq!("3", chain.next().unwrap().to_string());
    assert_eq!(3, chain.len());
    assert_eq!((3, Some(3)), chain.size_hint());
    assert_eq!("0", chain.next_back().unwrap().to_string());
    assert_eq!(2, chain.len());
    assert_eq!((2, Some(2)), chain.size_hint());
    assert_eq!("2", chain.next().unwrap().to_string());
    assert_eq!(1, chain.len());
    assert_eq!((1, Some(1)), chain.size_hint());
    assert_eq!("1", chain.next_back().unwrap().to_string());
    assert_eq!(0, chain.len());
    assert_eq!((0, Some(0)), chain.size_hint());
    assert!(chain.next().is_none());
}

#[test]
fn default() {
    let mut c = Chain::default();
    assert!(c.next().is_none());
}

#[test]
fn clone() {
    let e = error();
    let mut chain = e.chain().clone();
    assert_eq!("3", chain.next().unwrap().to_string());
    assert_eq!("2", chain.next().unwrap().to_string());
    assert_eq!("1", chain.next().unwrap().to_string());
    assert_eq!("0", chain.next().unwrap().to_string());
    assert!(chain.next().is_none());
    assert!(chain.next_back().is_none());
}

fn error() -> Error {
    narrate::error_from!({ 0 }).wrap(1).wrap(2).wrap(3)
}
