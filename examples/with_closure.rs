use ref_kman::Ref;

fn main() {
    let data = Ref::new(0);

    /// in this scope can be modificated
    data.mut_scope(|data| {
        *data = 21;
    });

    println!("Data: {}", data);
    assert_eq!(*data, 21);

    let a = 43;
    /// if you want to move a variabile to closure you need to make a other variabile a reference and put that
    let tmp_a = &a;
    ///move is to move the tmp_a to closure, this is why you dont want to move the original value because will be droped when closure ends!
    data.mut_scope(move |data| {
        let a = tmp_a;
        *data = *a;
    });

    println!("{}", data);
    assert_eq!(*data, a);
}
