use varnish::run_vtc_tests;
run_vtc_tests!("tests/*.vtc");

#[varnish::vmod(docs = "API.md")]
mod rs_example {
    ///This will tell you if a number is even, isn't that odd?
    pub fn is_even(
        /// the number to test
        n: i64,
    ) -> bool {
        // we could skip the return, or even use n.is_even(), but let's pace ourselves
        return n % 2 == 0;
    }

    /// Produce a string explaining which number you provided as argument
    pub fn captain_obvious(
        /// give us a number, or don't, it's optional!
        opt: Option<i64>,
    ) -> String {
        match opt {
            // no need to return, we are the last expression of the function!
            None => String::from("I was called without an argument"),
            // pattern matching FTW!
            Some(n) => format!("I was given {} as argument", n),
        }
    }
}

// Write some more unit tests
#[test]
fn obviousness() {
    assert_eq!(
        "I was called without an argument",
        rs_example::captain_obvious(None)
    );
    assert_eq!(
        "I was given 975322 as argument",
        rs_example::captain_obvious(Some(975322))
    );
}

// Write some more unit tests
#[test]
fn even_test() {
    assert_eq!(true, rs_example::is_even(0));
    assert_eq!(true, rs_example::is_even(1024));
    assert_eq!(false, rs_example::is_even(421321));
}
