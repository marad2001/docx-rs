/*
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static HEADER_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_header_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = HEADER_ID.load(Ordering::Relaxed);
    HEADER_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    id
}

#[cfg(test)]
pub fn generate_header_id() -> usize {
    123
}
*/
pub fn create_header_rid(id: usize) -> String {
    format!("rIdHeader{}", id)
}

pub fn parse_header_rid_index(rid: &str) -> Option<usize> {
    rid.strip_prefix("rIdHeader").and_then(|s| s.parse().ok())
}
