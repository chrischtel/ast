use fitsio::FitsFile;

pub fn open_fits_file(filename: &str) -> FitsFile {
    FitsFile::open(filename).unwrap()
}

#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}
