use std::fmt;
enum Gem {
    Diamond,
    Ruby,
    Sapphire,
    Topaz,
    Onyx,
    Jade,
}
impl fmt::Display for Gem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f,"test")
  }
    
}
fn main(){
  let gems=[
    (Gem::Onyx,25.00),
    (Gem::Ruby,10.00),
    (Gem::Sapphire,15.00),
    (Gem::Topaz,10.00),
  ];
  for (gem,price) in gems{
    println!("{}:${}",gem,price);
  }}
