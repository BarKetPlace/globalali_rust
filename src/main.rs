#[macro_use]
#[allow(unused)]
mod utils;
use utils::{Dtype,gamma,sfun,Params};


#[macro_use]
#[allow(unused)]
extern crate ndarray;
use ndarray::{Array};

#[allow(unused)]
fn main(){
    // Initialization
    let opt= Params{m:2,s:3,d:1,e:3};
    
    let s1 = "agta".as_bytes();
    let s2 = "ata".as_bytes();

    let n1 = s1.len() + 1;
    let n2 = s2.len() + 1;
    
    let mut v = Array::from_elem((n1,n2), 0. as Dtype);
    let mut f = Array::from_elem((n1,n2), 0. as Dtype);
    let mut g = Array::from_elem((n1,n2), 0. as Dtype);
    let mut h = Array::from_elem((n1,n2), 0. as Dtype);

    

    for i in 0..n1 {
        v[[i,0]] = gamma(i as i32, opt.d, opt.e);
    }
    for j in 0..n2 {
        v[[0,j]] = gamma(j as i32, opt.d, opt.e);
    }

    for i in (1..n1)  {
        for j in (1..n2) {
            f[[i,j]] = v[[i-1,j-1]] + sfun(s1[i-1] as char, s2[j-1] as char, opt.m, opt.s);
            g[[i,j]] = (v[[i-1,j]] - opt.d).max(g[[i-1,j]] - opt.e);
            h[[i,j]] = (v[[i,j-1]] - opt.d).max(h[[i,j-1]] - opt.e);
            v[[i,j]] = f[[i,j]].max(g[[i,j]]).max(h[[i,j]])
        } 
    }
    

    println!("{}", v);

   }

