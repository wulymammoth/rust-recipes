use std::io::Write;

#[allow(dead_code)]
pub struct Logger<T: Write> {
    out: T,
}

#[allow(dead_code)]
impl<T: Write> Logger<T> {
    pub fn new(out: T) -> Self {
        Logger { out }
    }

    // write messages directly to the given output with a newline
    pub fn log(&mut self, message: &str) {
        self.out.write(message.as_bytes()).unwrap();
        self.out.write(b"\n").unwrap();
    }

    // not useful ATM, unless there's buffering involved
    pub fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}

#[test]
fn test_mutable_reference() {
    // prepare our output stub
    let mut out = Vec::new();

    // don't take ownership, such that we can access it for assertion
    let mut logger = Logger::new(&mut out);
    logger.log("some warning");
    logger.flush();

    // easier to compare if we convert bytes into a string first
    let string_out = String::from_utf8(out).unwrap();

    assert_eq!(string_out, "some warning\n");
}

//#[test]
//#[ignore]
//fn test_mutable_reference_in_more_than_one_place() {
//let mut out = Vec::new();

//// will not compile because we can't take two mutable refs to the same vector
//// * we can def do this in another scope, but that's not the functionality we want or what we
//// want to test
//let mut logger1 = Logger::new(&mut out);
//let mut logger2 = Logger::new(&mut out); // can't take two mutable refs to the same vector

//logger1.log("one");
//logger2.log("two");
//logger1.log("three");

//logger1.flush();
//logger2.flush();

//assert_eq!(String::from_utf8(out).unwrap(), "one\ntwo\nthree\n");
//}

/// ROBUST STUBBING

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct TestWriter {
    storage: Rc<RefCell<Vec<u8>>>,
}

#[allow(dead_code)]
impl TestWriter {
    // creating a new `TestWriter` just means packaging an empty `Vec` in all the wrappers
    fn new() -> Self {
        TestWriter {
            storage: Rc::new(RefCell::new(Vec::new())),
        }
    }

    // once we are done writing to the buffer, we can take it (vector) out of the `Rc` and the
    // `RefCell` and inspect its contents
    fn into_inner(self) -> Vec<u8> {
        Rc::try_unwrap(self.storage).unwrap().into_inner()
    }

    // it's easier to compare strings instead of byte vectors
    fn into_string(self) -> String {
        let bytes = self.into_inner();
        String::from_utf8(bytes).unwrap()
    }
}

// the above struct is cloneable and holds a simple mutable vector
// all we need to do is implement the `Write` trait for it to delegate methods to the vector
// through mutable borrows, i.e., `burrow_mut()`
impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.storage.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.storage.borrow_mut().flush()
    }
}

// now, we can clone our "faux stdout" between two loggers without issue
#[test]
fn test_cloneable_writer() {
    let out = TestWriter::new();

    {
        let mut logger1 = Logger::new(out.clone());
        let mut logger2 = Logger::new(out.clone());
        logger1.log("one");
        logger2.log("two");
        logger1.log("three");

        logger1.flush();
        logger2.flush();
    }

    assert_eq!(out.into_string(), "one\ntwo\nthree\n");
}

// TODO: finish
//use std::sync::{Arc, Mutex};

//// thread-safe
//struct FauxOut {
    //storage: Arc<Mutex<Vec<u8>>>,
//}

//impl FauxOut {
    //// creating a new `TestWriter` just means packaging an empty `Vec` in all the wrappers
    //fn new() -> Self {
        //Self {
            //storage: Arc::new(Mutex::new(Vec::new())),
        //}
    //}

    //// once we are done writing to the buffer, we can take it (vector) out of the `Rc` and the
    //// `RefCell` and inspect its contents
    //fn into_inner(self) -> Vec<u8> {
        //Rc::try_unwrap(self.storage).unwrap().into_inner()
    //}

    //// it's easier to compare strings instead of byte vectors
    //fn into_string(self) -> String {
        //let bytes = self.into_inner();
        //String::from_utf8(bytes).unwrap()
    //}
//}

//impl Write for FauxOut {
    //fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        //self.storage.borrow_mut().write(buf)
    //}

    //fn flush(&mut self) -> std::io::Result<()> {
        //self.storage.borrow_mut().flush()
    //}
//}
