extern crate hextermbiest;

use hextermbiest::Hex;

#[test]
fn test_from_str() {
    let pairs = vec![
        ("#005fff", ("00", "5f", "ff")),
        ("#ff8787", ("ff", "87", "87")),
        ("#ff87af", ("ff", "87", "af")),
        ("#ffd75f", ("ff", "d7", "5f")),
    ];


    for (a, b) in pairs {
        assert_eq!(a.parse::<Hex>().unwrap().raw(), b);
    }
}
