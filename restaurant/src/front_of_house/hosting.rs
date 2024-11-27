// Also each item in the module is private by default. so we have to make it public to expose it to other 
// modules to use it without any error. 
pub fn add_to_waitlist() {}
fn seat_at_table() {} // This is private to the hosting module which other can not use.