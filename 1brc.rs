


#![feature(portable_simd)]
#![feature(slice_split_once)]



extern crate libc;
use libc::c_int;
use libc::c_void;
use libc::size_t;

use std::fs::File;
use std::os::fd::AsRawFd;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::simd::u8x64;
use std::simd::cmp::SimdPartialEq;



fn main() {
	// making keys as &[u8] may break MADV_SEQUENTIAL
	// TODO: measure
	let mut stats: HashMap<&[u8], (i16, i64, usize, i16)> = HashMap::new();

	let file = File::open("data/big_measurements.txt").unwrap();
	let file_map = unsafe { mmap(&file) };

	let mut at = 0;
	loop {
		let rest = &file_map[at..];
		let next_newline = unsafe { libc::memchr(
			rest.as_ptr() as *const c_void,
			b'\n' as c_int,
			rest.len()
		)};
		let line = if next_newline.is_null() {
			rest
		} else {
			let len = unsafe { (next_newline as *const u8).offset_from(rest.as_ptr()) } as usize;
			&rest[..len]
		};
		at += line.len() + 1;
		if line.is_empty() { break; }

		let (station, temperature) = if line.len() > 64 {
			line.rsplit_once(|c| *c == b';').unwrap()
		} else {
			let index_of_delimiter = {
				let delimiter = u8x64::splat(b';');
				let line = u8x64::load_or_default(line);
				let delim_eq = delimiter.simd_eq(line);
				unsafe { delim_eq.first_set().unwrap_unchecked() }
			};
			(&line[..index_of_delimiter], &line[index_of_delimiter + 1..])
		};
		let stats = stats.entry(station).or_insert((i16::MAX, 0, 0, i16::MIN));
		let temperature = parse_temperature(temperature);
		stats.0 = stats.0.min(temperature);
		stats.1 += i64::from(temperature);
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
		print!("{station}={:.1}/{:.1}/{:.1}",
			(f32::from(min) / 10.),
			(sum as f64) / (10. * count as f64),
			(f32::from(max) / 10.)
		);
		if stats.peek().is_some() {
			print!(", ");
		}
	}
	print!("}}");
}



fn parse_temperature(bytes: &[u8]) -> i16 {
	let mut temperature: i16 = 0;
	let mut mul = 1;
	for &d in bytes.iter().rev() {
		match d {
			b'.' => continue,
			b'-' => {
				temperature = -temperature;
				break;
			},
			_ => {
				temperature += i16::from(d - b'0') * mul;
				mul *= 10;
			}
		}
	}
	temperature
}



unsafe fn mmap(file: &File) -> &[u8] {
	let len = file.metadata().unwrap().len();
	let ptr = libc::mmap(
		std::ptr::null_mut(),
		len as size_t,
		libc::PROT_READ,
		libc::MAP_SHARED,
		file.as_raw_fd(),
		0
	);

	if ptr == libc::MAP_FAILED {
		panic!("{:?}", std::io::Error::last_os_error());
	}

	if libc::madvise(ptr, len as size_t, libc::MADV_SEQUENTIAL) != 0 {
		panic!("{:?}", std::io::Error::last_os_error());
	}

	std::slice::from_raw_parts(ptr as *const u8, len as usize)
}



