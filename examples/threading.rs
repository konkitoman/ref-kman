use ref_kman::Ref;

fn main() {
    let data = Ref::new(0);
    let tmp_data = data.clone();

    let thread_1 = std::thread::spawn(move || {
        let data = tmp_data;
        for _ in 0..10000000 {
            let mut data = data.get_mut();
            *data += 1;
        }
    });

    let tmp_data = data.clone();

    let thread_2 = std::thread::spawn(move || {
        let data = tmp_data;
        for _ in 0..10000000 {
            data.mut_scope(|data| {
                *data += 1;
            })
        }
    });

    let tmp_data = data.clone();

    let thread_3 = std::thread::spawn(move || {
        let data = tmp_data;
        loop {
            if *data == 20000000 {
                break;
            }
            println!("D: {}", data);
        }
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();

    println!("Data: {}", data)
}
