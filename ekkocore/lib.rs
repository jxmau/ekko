pub mod nature;
pub mod query;

pub fn debug(query: &str) {

    let buf = Buffer::new();

    // Determine the Nature of it
    let n = Nature::new(&buf, query);

    // Create the query
    // Debug the query

}