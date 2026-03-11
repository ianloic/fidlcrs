
use crate::tests::test_library::{SharedAmongstLibraries, TestLibrary};

#[test]
fn error0056() {
    // Interleaving availability test
    let mut shared = SharedAmongstLibraries::new();
    let mut dep = TestLibrary::with_shared(&mut shared);
    dep.add_source_file("dep.fidl", "library dependent;
@available(added=1)
type Bar = struct {};");
    dep.compile().expect("dep compiled");

    let mut lib = TestLibrary::with_shared(&mut shared);
    lib.add_source_file("example.fidl", "library example;
using dependent;
@available(added=2)
type Foo = struct { b dependent.Bar; };");
    lib.compile().expect("lib compiled");
}
