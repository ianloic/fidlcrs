#![allow(unused_mut)]
use crate::tests::test_library::TestLibrary;

#[test]
fn good0001() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0001.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0002() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0002.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0003() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0003.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0004() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0004.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0006() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0006.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0007() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0007.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0008() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0008.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0009() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0009.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0010a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0010-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0010b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0010-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0011() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0011.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0012() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0012.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0013() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0013.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0014() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0014.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0015() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0015.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0016() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0016.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0017() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0017.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0018() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0018.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0019a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0019-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0019b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0019-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0020() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0020.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0022() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0022.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0023() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0023.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0024() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0024.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0025() {
    let mut shared = TestLibrary::new();
    // TODO: TestLibrary dependency(&shared, "dependent.fidl", R"FIDL(
    // TODO: library dependent;
    // TODO: type Something = struct {
}

#[test]
fn good0026() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0026.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0027a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0027-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0027b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0027-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0028a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0028-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0028b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0028-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0029() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0029.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0030() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0030.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0031() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0031.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0032() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0032.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0033() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0033.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0034a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0034-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0034b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0034-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0035() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0035.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0036() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0036.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0037() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0037.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0038ab() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0038-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0038-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0038ac() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0038-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0038-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0039ab() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0039-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0039-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0039ac() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0039-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0039-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0040() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0040-a.test.fidl");
    library.add_errcat_file("good/fi-0040-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0041a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0041-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0041b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0041-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0042() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0042-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0042-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0043() {
    let mut shared = TestLibrary::new();
    let mut dependency1 = TestLibrary::new();
    dependency1.add_errcat_file("good/fi-0043-a.test.fidl");
    let _ = dependency1.compile().expect("compilation failed");
    let mut dependency2 = TestLibrary::new();
    dependency2.add_errcat_file("good/fi-0043-b.test.fidl");
    let _ = dependency2.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0043-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0044() {
    let mut shared = TestLibrary::new();
    let mut dependency1 = TestLibrary::new();
    dependency1.add_errcat_file("good/fi-0044-a.test.fidl");
    let _ = dependency1.compile().expect("compilation failed");
    let mut dependency2 = TestLibrary::new();
    dependency2.add_errcat_file("good/fi-0044-b.test.fidl");
    let _ = dependency2.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0044-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0045() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0045-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0045-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0046() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0046.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0048() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0048.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0049() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0049.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0050() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0050.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0051() {
    let mut shared = TestLibrary::new();
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0051-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0051-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0052a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0052-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0052b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0052-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0053a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0053-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0053b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0053-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0054() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0054.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0055() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0055.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0056() {
    let mut shared = TestLibrary::new();
    shared.select_version("foo", "HEAD");
    shared.select_version("bar", "HEAD");
    let mut dependency = TestLibrary::new();
    dependency.add_errcat_file("good/fi-0056-a.test.fidl");
    let _ = dependency.compile().expect("compilation failed");
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0056-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0057() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0057.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0058() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0058.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0059() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0059.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0060() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0060.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0061() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0061.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0062a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0062-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0062b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0062-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0063() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0063.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0064a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0064-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0064b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0064-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0065a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0065-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0065b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0065-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0065c() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0065-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0066a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0066-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0066b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0066-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0067a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0067-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0067b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0067-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0068a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0068-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0068b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0068-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0069() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0069.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0070() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0070.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0071a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0071-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0071b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0071-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0072a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0072-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0072b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0072-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0073() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0073.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0074() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0074.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0075() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0075.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0077a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0077-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0077b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0077-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0081() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0081.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0082() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0082.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0083() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0083.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0084() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0084.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0088() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0088.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0091() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0091.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0092() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0092.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0093() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0093.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0094a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0094-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0094b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0094-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0097a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0097-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0097b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0097-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0101() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0101.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0102() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0102.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0103() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0103.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0104() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0104.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0107a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0107-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0107b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0107-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0110a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0110-a.test.fidl");
    library.use_library_zx();
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0110b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0110-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0111() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0111.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0112() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0112.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0113() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0113.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0114a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0114-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0114b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0114-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0115a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0115-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0115b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0115-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0116a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0116-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0116b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0116-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0117a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0117-a.test.fidl");
    library.use_library_zx();
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0117b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0117-b.test.fidl");
    // library.UseLibraryFdf();
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0118() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0118.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0120a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0120-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0120b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0120-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0121() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0121.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0122() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0122.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0123() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0123.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0124() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0124.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0125() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0125.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0126() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0126.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0127() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0127.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0128() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0128.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0129a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0129-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0129b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0129-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0130() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0130.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0131a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0131-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0131b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0131-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0132() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0132.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0133() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0133.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0135() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0135.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0141() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0141.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0142() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0142.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0145() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0145.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0146() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0146.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0147() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0147.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0148a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0148-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0148b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0148-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0149a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0149-a.test.fidl");
    library.select_version("foo", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0149b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0149-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0150a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0150-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0150b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0150-b.test.fidl");
    library.select_version("foo", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0151a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0151-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0151b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0151-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0152() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0152.test.fidl");
    library.select_version("foo", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0153() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0153.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0154a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0154-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0154b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0154-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0155() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0155.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0156() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0156.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0157() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0157.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0158() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0158.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0159() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0159.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0160a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0160-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0160b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0160-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0161() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0161.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0162() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0162.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0163() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0163.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0164() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0164.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0165() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0165.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0166() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0166.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0167() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0167.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0168() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0168.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0169() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0169.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0171() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0171.test.fidl");
    library.use_library_zx();
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0172() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0172.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0173() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0173.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0175() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0175.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0177() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0177.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0178() {
    let mut shared = TestLibrary::new();
    // TODO: TestLibrary dependency(&shared, "dependent.fidl", R"FIDL(
    // TODO: library dependent;
    // TODO: type Bar = struct {
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0179() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0179.test.fidl");
    // library.EnableFlag(ExperimentalFlag::kAllowNewTypes);
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0180() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0180.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0181() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0181.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0184a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0184-a.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0184b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0184-b.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0185() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0185.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0186() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0186.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0187() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0187.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0188() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0188.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0189() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0189.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0191() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0191.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0192() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0192.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0193() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0193.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0201() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0201.test.fidl");
    library.select_version("foo", "1");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0203a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0203-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0203b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0203-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0204() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0204.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0205a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0205-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0205b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0205-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0205c() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0205-c.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0206a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0206-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0206b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0206-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0207() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0207.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0208() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0208.test.fidl");
    library.select_version("foo", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0209a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0209-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0209b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0209-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0209c() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0209-c.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0210() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0210.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0211() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0211.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0212a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0212-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0212b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0212-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0213a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0213-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0213b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0213-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0214a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0214-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0214b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0214-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0215() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0215.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0216a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0216-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0216b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0216-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0217a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0217-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0217b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0217-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0217c() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0217-c.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0218() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0218.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0219a() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0219-a.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
#[ignore] // TODO: Fix missing compiler features
fn good0219b() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0219-b.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0220() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0220.test.fidl");
    library.select_version("test", "HEAD");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0221() {
    let mut library = TestLibrary::new();
    // library.experimental_flags().Enable(ExperimentalFlag::kNoResourceAttribute);
    library.add_errcat_file("good/fi-0221.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0222() {
    let mut library = TestLibrary::new();
    library.add_errcat_file("good/fi-0222.test.fidl");
    let _ = library.compile().expect("compilation failed");
}

#[test]
fn good0223() {
    let mut library = TestLibrary::new();
    // library.experimental_flags().Enable(ExperimentalFlag::kNoResourceAttribute);
    library.add_errcat_file("good/fi-0223.test.fidl");
    let _ = library.compile().expect("compilation failed");
}
