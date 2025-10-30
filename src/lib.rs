pub mod nes;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

//pub fn load_rom_mmap(&mut self, path: impl AsRef<Path>) -> Result<&mut Self>


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
