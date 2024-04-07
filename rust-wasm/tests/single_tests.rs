use rust_wasm::OpSeq;

#[test]
fn compose_operation() {
    let mut a = OpSeq::default();
    a.insert("abc");
    let mut b = OpSeq::default();
    b.retain(3);
    b.insert("def");
    let after_a = a.apply("").unwrap();
    let after_b = b.apply(&after_a).unwrap();
    let after_ab = a.compose(&b).unwrap().apply("").unwrap();
    assert_eq!(after_ab, after_b);
}

#[test]
fn transform_operations() {
    let s = "abc";
    let mut a = OpSeq::default();
    a.retain(3);
    a.insert("def");
    let mut b = OpSeq::default();
    b.retain(3);
    b.insert("ghi");
    // receive pair tuple
    let pair = a.transform(&b).unwrap();
    let (a_prime, b_prime) = (pair.first(), pair.second());
    let ab_prime = a.compose(&b_prime).unwrap();
    let ba_prime = b.compose(&a_prime).unwrap();
    let after_ab_prime = ab_prime.apply(s).unwrap();
    let after_ba_prime = ba_prime.apply(s).unwrap();
    assert_eq!(ab_prime, ba_prime);
    assert_eq!(after_ab_prime, after_ba_prime);
}

#[test]
fn test_apply() {
    let initial_state = "abc";
    let mut op = OpSeq::default();
    // need to retain origin string  (1, n - 1), (2, n - 2), ...
    op.retain(1);
    op.insert("ghi");
    op.retain(2);
    let result = op.apply(initial_state).unwrap();
    // abghic
    assert_eq!(result, "aghibc".to_string());
}

#[test]
fn invert_operations() {
    let s = "abc";
    let mut o = OpSeq::default();
    o.retain(3);
    o.insert("def");
    let p = o.invert(s);
    assert_eq!(p.apply(&o.apply(s).unwrap()).unwrap(), s);
}
