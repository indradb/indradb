// We use a macro to avoid take_while, and the overhead that closure callbacks would cause
macro_rules! prefix_iterate {
	($manager:expr, $prefix:expr, $key:ident, $value:ident, $code:block) => {
		for ($key, $value) in try!($manager.db.iterator_cf($manager.cf, IteratorMode::From($prefix, Direction::Forward))) {
			if !$key.starts_with($prefix) {
				break;
			}

			$code;
		}
	}
}

macro_rules! reverse_iterate {
	($manager:expr, $max_key:expr, $prefix:expr, $key:ident, $value:ident, $code:block) => {
		for ($key, $value) in try!($manager.db.iterator_cf($manager.cf, IteratorMode::From($max_key, Direction::Reverse))) {
			if !$key.starts_with($prefix) {
				break;
			}

			$code;
		}
	}
}