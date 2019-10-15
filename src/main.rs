#[macro_use]
#[allow(unused)]
mod utils;
use utils::{Dtype,gamma,sfun,Params};

#[macro_use]
#[allow(unused)]
extern crate ndarray;
use ndarray::{Array};
extern crate clap;
use clap::{App, Arg};

#[allow(unused)]
fn main(){
    let matches = App::new("GlobalAli\nDefine: Gamma(n) = (d-e) + n*e")
        .arg(
            Arg::with_name("input")
            .short("i")
            .long("input-seq")
            .help("input sequences")
            .required(true)
            .multiple(true)
            .number_of_values(2)
            )
           .arg(
            Arg::with_name("d param")
            .short("d")
            .long("dparam")
            .help( "(d-e) Gamma function intercept ")
            .default_value("1")
            .required(false)
        )
    .arg(
            Arg::with_name("s param")
            .short("s")
            .long("sparam")
            .help( "Different elts.")
            .default_value("2")
            .required(false)
        )
    .arg(
            Arg::with_name("m param")
            .short("m")
            .long("mparam")
            .help( "Value for Similar elts.")
            .default_value("3")
            .required(false)
        )
    .arg(
            Arg::with_name("e param")
            .short("e")
            .long("eparam")
            .help( "Gamma function steep")
            .default_value("1")
            .required(false)
        )
        .get_matches();
    
    // Initialization
    let opt= Params{
        m: matches.value_of("m param").unwrap().parse::<Dtype>().unwrap(),
        s: matches.value_of("s param").unwrap().parse::<Dtype>().unwrap(),
        d: matches.value_of("d param").unwrap().parse::<Dtype>().unwrap(),
        e: matches.value_of("e param").unwrap().parse::<Dtype>().unwrap(),
    };
    
    let s:Vec<&str> = matches.values_of("input").unwrap().collect();
    let s1 = s[0].as_bytes();
    let s2 = s[1].as_bytes();

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

