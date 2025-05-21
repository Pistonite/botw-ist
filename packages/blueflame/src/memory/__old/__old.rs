

// impl Memory {
//     /// Returns a String representing the string representation of the SafeString stored at address ptr
//     pub fn mem_read_safe_string(&mut self, ptr: u64) -> Result<String, Error> {
//         let m_string_top_addr = self.mem_read_u64(ptr + 8)?;
//         let mut addr = m_string_top_addr;
//         let mut byte = self.mem_read_byte(addr)?;
//         let mut bytes = Vec::<u8>::new();
//         while byte != 0 {
//             bytes.push(byte);
//             byte = self.mem_read_byte(addr)?;
//             addr += 1;
//         }
//         let s = str::from_utf8(&bytes).map_err(|_e| {
//             crate::memory::Error::Unexpected(String::from("Unexpected error reading SafeString"))
//         })?;
//         Ok(String::from(s))
//     }
// }
