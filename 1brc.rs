


use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::FromIterator;



fn main() {
	let mut stats: HashMap<String, (f64, f64, usize, f64)> = HashMap::new();

	let file = File::open("data/big_measurements.txt").unwrap();
	let file = BufReader::new(file);

	for line in file.lines() {
		let line = line.unwrap();
		let (station, temperature) = line.split_once(';').unwrap();
		let temperature: f64 = temperature.parse().unwrap();
		let stats = match stats.get_mut(station) {
			Some(stats) => stats,
			None => stats
				.entry(station.to_string())
				.or_insert((f64::MAX, 0., 0, f64::MIN))
		};
		stats.0 = stats.0.min(temperature);
		stats.1 += temperature;
		stats.2 += 1;
		stats.3 = stats.3.max(temperature);
	}

	print!("{{");
	let stats = BTreeMap::from_iter(stats);
	let mut stats = stats.into_iter().peekable();
	while let Some((station, (min, sum, count, max))) = stats.next() {
		print!("{station}={min:.1}/{:.1}/{max:.1}", sum/(count as f64));
		if stats.peek().is_some() {
			print!(", ");
		}
	}
	print!("}}");
}



