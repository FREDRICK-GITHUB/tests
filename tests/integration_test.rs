/*this file does not need the test annotation because rust will treat it as such by default
Integration tests will go in here */
use adder::*;

#[test]
fn it_adds_two () {
    assert_eq!(4,add_two(2));
}