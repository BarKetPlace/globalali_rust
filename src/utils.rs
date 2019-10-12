pub fn gamma(n:i32,d:Dtype,e:Dtype) -> Dtype {
  return d + (n as Dtype - (1. as Dtype)) * e;
}

#[test]
fn test_gamma() {
  assert_eq!(gamma(0), 1.)
}

pub type Dtype = i32;

pub struct Params {
  pub m:Dtype,
  pub s:Dtype,
  pub d:Dtype,
  pub e:Dtype
}


pub fn sfun(x:char, y:char,m:Dtype,s:Dtype) -> Dtype {
  if x == y {
    return m;
  } 
  else {
    return -s;
  }
}
