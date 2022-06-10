use aidoku::{prelude::format, std::String};

pub fn listing_map(listing: String) -> String {
	let url: &str = match listing.as_str() {
		"Popular" => "popular-comics",
		"Hot" => "hot",
		"Completed" => "status/completed",
		"Ongoing" => "status/ongoing",
		_ => "",
	};
	return String::from(url);
}

// MARK: Other utilities
pub fn get_search_url(base_url: String, query: String, genre: String, page: i32) -> String {
	return if query.len() > 0 {
		format!("{base_url}/search?page={page}&keyword={query}")
	} else if genre.len() > 0 {
		format!("{base_url}/genre/{genre}?page={page}")
	} else {
		base_url
	};
}