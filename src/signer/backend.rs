pub trait SigningBackend {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
    fn public_key(&self) -> Vec<u8>;
}
