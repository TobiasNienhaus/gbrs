
/// Check a specific bit in the byte. Returns true, if the byte is set. 0-indexed (first bit is 0, last is 7)
pub fn check_bit(val: u8, bit: u8) -> bool {
    ((val >> bit) & 0b1) == 0b1
}