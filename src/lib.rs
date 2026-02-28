use std::io;

use io::BufRead;
use io::Seek;

use io::Cursor;
use io::Read;

use io::BufWriter;
use io::Write;

use zune_jpeg::JpegDecoder;

pub fn reader2jpg2exif2writer<R, W>(rdr: R, mut wtr: W) -> Result<(), io::Error>
where
    R: BufRead + Seek,
    W: Write,
{
    let mut dec = JpegDecoder::new(rdr);
    dec.decode().map_err(io::Error::other)?;
    let oexif: Option<_> = dec.exif();
    match oexif {
        None => Ok(()),
        Some(exif) => {
            wtr.write_all(exif)?;
            wtr.flush()
        }
    }
}

pub fn img_bytes2jpg2exif2writer<W>(img_bytes: &[u8], wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let cur = Cursor::new(img_bytes);
    reader2jpg2exif2writer(cur, wtr)
}

pub fn reader2limited2jpg2exif2writer<R, W>(limit: u64, rdr: R, wtr: W) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
{
    let mut taken = rdr.take(limit);
    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;
    img_bytes2jpg2exif2writer(&buf, wtr)
}

pub fn stdin2jpg2exif2stdout(limit: u64) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    reader2limited2jpg2exif2writer(limit, io::stdin().lock(), BufWriter::new(&mut ol))?;
    ol.flush()
}

pub const IMG_INPUT_SIZE_LIMIT_DEFAULT: u64 = 16777216;

pub fn stdin2jpg2exif2stdout_default() -> Result<(), io::Error> {
    stdin2jpg2exif2stdout(IMG_INPUT_SIZE_LIMIT_DEFAULT)
}
