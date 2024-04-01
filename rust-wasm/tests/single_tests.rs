use operational_transform::OperationSeq;

#[test]
fn compose_operation() {
    let mut a = OperationSeq::default();
    a.insert("abc");
    let mut b = OperationSeq::default();
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
    let mut a = OperationSeq::default();
    a.retain(3);
    a.insert("def");
    let mut b = OperationSeq::default();
    b.retain(3);
    b.insert("ghi");
    let (a_prime, b_prime) = a.transform(&b).unwrap();
    let ab_prime = a.compose(&b_prime).unwrap();
    let ba_prime = b.compose(&a_prime).unwrap();
    let after_ab_prime = ab_prime.apply(s).unwrap();
    let after_ba_prime = ba_prime.apply(s).unwrap();
    assert_eq!(ab_prime, ba_prime);
    assert_eq!(after_ab_prime, after_ba_prime);
}

#[test]
fn invert_operations() {
    let s = "abc";
    let mut o = OperationSeq::default();
    o.retain(3);
    o.insert("def");
    let p = o.invert(s);
    assert_eq!(p.apply(&o.apply(s).unwrap()).unwrap(), s);
}