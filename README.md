# Memcached Client For Fun

#####  A Memcached client implementation in Rust

Example:

    let mut memcached_init = Memcached { host: String::from("127.0.0.1"), port: 11211};
    let mut memcached_connection = memcached_init.connect().unwrap();


    let key = String::from("foo");
    let val = String::from("abc\nd\ndef");

    let write_response = memcached_connection.write(key.clone(), val, Some(10));

    match write_response {
        Ok(_) => println!("wrote"),
        Err(err) => println!("{}", err),
    }

    let read_response = memcached_connection.read(key.clone());

    match read_response {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("{}", err),
    }

    let deleted_response = memcached_connection.delete(key.clone());

    match deleted_response {
        Ok(_) => println!("deleted"),
        Err(err) => println!("err: {}", err),
    }
