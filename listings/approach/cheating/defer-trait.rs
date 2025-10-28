impl Deref for StructC { type Target = StructA;
  fn deref(&self) -> &Self::Target { &self.0 }}
  fn main() {
    StructC::new().method();
}
