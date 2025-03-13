use rug::{Assign, Integer};
use std::error::Error;
use std::io::Write;
use std::net::TcpListener;
use threadpool::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = ThreadPool::new(5);
    let listener = TcpListener::bind("0.0.0.0:31415")?;

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            if pool.active_count() == pool.max_count() {
                stream.write(b"Pigen server is too busy! Please wait...\n")?;
            }

            pool.execute(move || {
                // This pi digit spigot algorithm is based on pseudocode from:
                // https://homepages.cwi.nl/~steven/abc/programmers/examples.html
                // (Section: Numbers, Subsection: Pi)

                let mut k = Integer::from(2);
                let mut a = Integer::from(4);
                let mut b = Integer::from(1);
                let mut a1 = Integer::from(12);
                let mut b1 = Integer::from(4);

                'pi: loop {
                    let p = Integer::from(&k * &k);
                    let q = Integer::from(2 * &k) + 1;
                    let prev_a1 = a1.clone();
                    let prev_b1 = b1.clone();

                    k += 1;
                    a1 *= &q;
                    a1 += &p * &a;
                    b1 *= &q;
                    b1 += &p * &b;
                    a = prev_a1;
                    b = prev_b1;

                    let mut d = Integer::from(&a / &b);
                    let mut d1 = Integer::from(&a1 / &b1);

                    while d == d1 {
                        if let Err(_e) = stream.write(d.to_string().as_ref()) {
                            break 'pi;
                        }

                        a %= &b;
                        a *= 10;
                        a1 %= &b1;
                        a1 *= 10;
                        d.assign(&a / &b);
                        d1.assign(&a1 / &b1);
                    }
                }
            });
        }
    }

    Ok(())
}
