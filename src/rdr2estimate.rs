use std::io;
use std::io::BufReader;
use std::io::Read;

use crate::bloom2estimate::BloomStat;

pub fn reader2estimate<R>(rdr: R, buf: &mut [u8], nhash: f64) -> Result<f64, io::Error>
where
    R: Read,
{
    let br = BufReader::new(rdr);
    let (ones, tot) = crate::rdr2ones::reader2ones(br, buf)?;
    let stat = BloomStat {
        num_bits: tot as f64,
        num_hash: nhash,
        num_ones: ones as f64,
    };
    Ok(stat.estimate_count())
}
