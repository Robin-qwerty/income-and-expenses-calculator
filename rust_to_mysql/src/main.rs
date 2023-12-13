extern crate mysql;
use mysql::*;
use mysql::prelude::*;

fn main() {
    let url = "mysql://rust-inc-exp:qwerty@localhost:3306/rust-inc-exp";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_iter("SELECT `id`, `name`, `password` FROM `users`")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, String) = from_row(row.unwrap());
            println!("{}, {}, {:?}", r.0, r.1, r.2);
        });

    // let res:Vec<(i32, String, NaiveDate)> = conn.query("SELECT `id`, `name`, `password` FROM `users`")
    //     .unwrap();
 
    // for r in res {
    //     println!("{}, {}, {:?}", r.0, r.1, r.2);
    // }

    // struct Users {
    //     id: u64,
    //     name: String,
    //     password: String
    // }
}

