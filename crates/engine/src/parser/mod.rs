pub mod html;
pub mod normalize;

pub use html::{parse_url_parts, select_item, PageJsonSources, UrlParts};
pub use normalize::{normalize_item, merge_oembed, evaluate_quality};
