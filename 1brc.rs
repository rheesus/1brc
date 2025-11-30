


extern crate libc;

use std::fs::File;
use std::os::fd::AsRawFd;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::FromIterator;



fn main() {
	let mut stats: HashMap<&[u8], (f64, f64, usize, f64)> = HashMap::new();

	let file = File::open("data/big_measurements.txt").unwrap();
	let file_map = unsafe { mmap(&file) };

	for line in file_map.split(|c| *c == b'\n') {
		if line.is_empty() { break; }
		let mut fields = line.rsplitn(2, |c| *c == b';');
		let temperature = fields.next().unwrap();
		let temperature: f64 = unsafe { str::from_utf8_unchecked(temperature) }.parse().unwrap();
		let station = fields.next().unwrap();
		let stats = stats.entry(station).or_insert((f64::MAX, 0., 0, f64::MIN));
		stats.0 = stats.0.min(temperature);
		stats.1 += temperature;
		stats.2 += 1;
		stats.3 = stats.3.max(temperature);
	}

	print!("{{");
	let stats = BTreeMap::from_iter(
		stats.into_iter()
			.map(|(station, v)| (unsafe { str::from_utf8_unchecked(station) }, v))
	);
	let mut stats = stats.into_iter().peekable();
	while let Some((station, (min, sum, count, max))) = stats.next() {
		print!("{station}={min:.1}/{:.1}/{max:.1}", sum/(count as f64));
		if stats.peek().is_some() {
			print!(", ");
		}
	}
	print!("}}");
}



unsafe fn mmap(file: &File) -> &[u8] {
	let len = file.metadata().unwrap().len();
	let ptr = libc::mmap(
		std::ptr::null_mut(),
		len as libc::size_t,
		libc::PROT_READ,
		libc::MAP_SHARED,
		file.as_raw_fd(),
		0
	);

	if ptr == libc::MAP_FAILED {
		panic!("{:?}", std::io::Error::last_os_error());
	}

	if libc::madvise(ptr, len as libc::size_t, libc::MADV_SEQUENTIAL) != 0 {
		panic!("{:?}", std::io::Error::last_os_error());
	}

	std::slice::from_raw_parts(ptr as *const u8, len as usize)
}



