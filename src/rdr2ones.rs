use std::io;
use std::io::Read;

pub fn reader2ones<R>(mut rdr: R, buf: &mut [u8]) -> Result<(u64, u64), io::Error>
where
    R: Read,
{
    let mut cnt: u64 = 0;
    let mut tot: u64 = 0;
    loop {
        buf.fill(0);
        let read: usize = rdr.read(buf)?;
        if 0 == read {
            return Ok((cnt, tot));
        }
        let s: &[u8] = &buf[..read];
        cnt += s.iter().fold(0, |state, next| {
            let u: u8 = *next;
            let tot: u32 = u.count_ones();
            state + u64::from(tot)
        });
        tot += (s.len() << 3) as u64;
    }
}
