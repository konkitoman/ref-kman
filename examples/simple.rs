use ref_kman::Ref;

fn main() {
    let data = Ref::new(0);

    {
        let mut mut_data = data.get_mut();
        println!("Is locked: {}", data.locked());

        *mut_data = 21;
    }

    // Dont do this if you do this the current thread will be stuck in a loop because noting will be droped to unlock;
    //let mut mut_data = data.get_mut();
    //let mut seccond_mut_data = data.get_mut();

    //Never do:
    // data.lock()
    // data.unlock()
    // this is very bed!
    // do this if you really know what are you doing

    let data2 = data.clone();
    // data2 is not only the value but is has the same pointer
    // if data2 is modificated will be applied to data

    data2.mut_scope(|value| {
        *value = 33;
    });

    // when all ref will be droped, then the data will be droped!

    println!("Simple: {}", data);
}
