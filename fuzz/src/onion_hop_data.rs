// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

// This file is auto-generated by gen_target.sh based on msg_target_template.txt
// To modify it, modify msg_target_template.txt and run gen_target.sh instead.

use crate::utils::test_logger;

#[inline]
pub fn onion_hop_data_test<Out: test_logger::Output>(data: &[u8], _out: Out) {
	use lightning::util::ser::Readable;
	let mut r = ::std::io::Cursor::new(data);
	let _ =  <lightning::ln::msgs::InboundOnionPayload as Readable>::read(&mut r);
}

#[no_mangle]
pub extern "C" fn onion_hop_data_run(data: *const u8, datalen: usize) {
	use lightning::util::ser::Readable;
	let data = unsafe { std::slice::from_raw_parts(data, datalen) };
	let mut r = ::std::io::Cursor::new(data);
	let _ =  <lightning::ln::msgs::InboundOnionPayload as Readable>::read(&mut r);
}
