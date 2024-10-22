use std::env;
use std::io;

use std::process::ExitCode;

use rs_bloom2count::rdr2estimate::reader2estimate;

fn stdin2stdout(buf: &mut [u8], nhash: f64) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let estimate: f64 = reader2estimate(il, buf, nhash)?;
    println!("{estimate}");
    Ok(())
}

fn sub() -> Result<(), io::Error> {
    let bufsz: usize = env::var("ENV_BUF_SIZE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(4096);
    let nhash: f64 = env::var("ENV_NUM_HASH")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(1.0);
    let mut buf: Vec<u8> = vec![0; bufsz];
    stdin2stdout(&mut buf, nhash)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
