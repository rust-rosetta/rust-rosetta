// Implements http://rosettacode.org/wiki/Four_bit_adder
#![allow(non_snake_case_functions)]

extern crate arena;

use arena::Arena;

static TRUE: bool = true;
static FALSE: bool = false;

trait Gate {
  fn get(&self) -> bool;
}

impl Gate for bool {
  fn get(&self) -> bool {
    *self
  }
}

struct NotGate<'a>{
  inp: &'a Gate,
}

impl<'a> Gate for NotGate<'a> {
 fn get(&self) -> bool {
    !self.inp.get()
  }
}

struct OrGate<'a>{
  inp1: &'a Gate,
  inp2: &'a Gate,
}

impl<'a> Gate for OrGate<'a> {
  fn get(&self) -> bool {
    self.inp1.get() || self.inp2.get()
  }
}

struct AndGate<'a>{
  inp1: &'a Gate,
  inp2: &'a Gate,
}

impl<'a> Gate for AndGate<'a> {
  fn get(&self) -> bool {
    self.inp1.get() && self.inp2.get()
  }
}

fn not<'a>(ar: &'a Arena, inp: &'a Gate) -> &'a Gate {
  ar.alloc(||NotGate{inp:inp}) as &'a Gate
}

fn or<'a>(ar: &'a Arena, inp1: &'a Gate, inp2: &'a Gate) -> &'a Gate {
  ar.alloc(||OrGate{inp1:inp1,inp2:inp2}) as &'a Gate
}

fn and<'a>(ar: &'a Arena, inp1: &'a Gate, inp2: &'a Gate) -> &'a Gate {
  ar.alloc(||AndGate{inp1:inp1,inp2:inp2}) as &'a Gate
}

fn xor<'a>(ar: &'a Arena, inp1: &'a Gate, inp2: &'a Gate) -> &'a Gate {
  or(ar,and(ar,inp1,not(ar,inp2)),and(ar,not(ar,inp1),inp2))
}

fn half_add<'a>(ar: &'a Arena, inp1: &'a Gate, inp2: &'a Gate) -> (&'a Gate, &'a Gate) {
  (xor(ar,inp1,inp2),and(ar,inp1,inp2))
}

fn full_add<'a>(ar: &'a Arena, inp1: &'a Gate, inp2: &'a Gate, inp3: &'a Gate)
    -> (&'a Gate, &'a Gate) {
  let (ha1_s,ha1_c) = half_add(ar,inp1, inp3);
  let (ha2_s,ha2_c) = half_add(ar,ha1_s,inp2);
  (ha2_s,or(ar,ha1_c,ha2_c))
}

type Nibble<'a> = (&'a Gate, &'a Gate, &'a Gate, &'a Gate);

fn four_bit_adder<'a>(ar: &'a Arena, a: Nibble<'a>, b: Nibble<'a>, ci: &'a Gate)
    -> (&'a Gate, Nibble<'a>) {
  let (a4,a3,a2,a1) = a;
  let (b4,b3,b2,b1) = b;
  let (fa1_s,fa1_c) = full_add(ar,a1,b1,ci);
  let (fa2_s,fa2_c) = full_add(ar,a2,b2,fa1_c);
  let (fa3_s,fa3_c) = full_add(ar,a3,b3,fa2_c);
  let (fa4_s,fa4_c) = full_add(ar,a4,b4,fa3_c);
  (fa4_c, (fa4_s,fa3_s,fa2_s,fa1_s))
}

#[cfg(not(test))]
fn main() {
  fn show_nibble<'a>(nib: Nibble<'a>) -> (bool,bool,bool,bool) {
    let (n4,n3,n2,n1) = nib;
    (n4.get(),n3.get(),n2.get(),n1.get())
  }
  fn show_result<'a>((c,nib): (&'a Gate,Nibble<'a>)) -> (bool,(bool,bool,bool,bool)) {
    (c.get(), show_nibble(nib))
  }
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  let inp1 = (gTrue,gFalse,gTrue,gTrue);
  let inp2 = (gFalse,gTrue,gTrue,gFalse);
  let (oflow,res) = show_result(four_bit_adder(ar,inp1,inp2,gFalse));
  println!("{} + {} = {}, overflow: {}", show_nibble(inp1), show_nibble(inp2), res, oflow);
}

#[test]
fn test_not() {
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  assert_eq!(true, not(ar,gFalse).get());
  assert_eq!(false, not(ar,gTrue).get());
}

#[test]
fn test_or() {
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  assert_eq!(false, or(ar,gFalse,gFalse).get());
  assert_eq!(true, or(ar,gTrue,gFalse).get());
  assert_eq!(true, or(ar,gFalse,gTrue).get());
  assert_eq!(true, or(ar,gTrue,gTrue).get());
}

#[test]
fn test_and() {
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  assert_eq!(false, and(ar,gFalse,gFalse).get());
  assert_eq!(false, and(ar,gFalse,gTrue).get());
  assert_eq!(false, and(ar,gTrue,gFalse).get());
  assert_eq!(true, and(ar,gTrue,gTrue).get());
}

#[test]
fn test_xor() {
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  assert_eq!(false, xor(ar,gFalse,gFalse).get());
  assert_eq!(true, xor(ar,gFalse,gTrue).get());
  assert_eq!(true, xor(ar,gTrue,gFalse).get());
  assert_eq!(false, xor(ar,gTrue,gTrue).get());
}

#[test]
fn test_full_add() {
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  fn eval((a,b): (&Gate,&Gate)) -> (bool,bool) { (a.get(),b.get()) }
  assert_eq!( (false,false), eval(full_add(ar,gFalse,gFalse,gFalse)) );
  assert_eq!( (true,false), eval(full_add(ar,gFalse,gFalse,gTrue)) );
  assert_eq!( (true,false), eval(full_add(ar,gFalse,gTrue,gFalse)) );
  assert_eq!( (true,false), eval(full_add(ar,gTrue,gFalse,gFalse)) );
  assert_eq!( (false,true), eval(full_add(ar,gFalse,gTrue,gTrue)) );
  assert_eq!( (false,true), eval(full_add(ar,gTrue,gFalse,gTrue)) );
  assert_eq!( (false,true), eval(full_add(ar,gTrue,gTrue,gFalse)) );
  assert_eq!( (true,true), eval(full_add(ar,gTrue,gTrue,gTrue)) );
}

#[test]
fn test_full_add4() {
 fn to_nib(n:u8)->(Nibble){
    let (gt,gf) = (&TRUE as &Gate, &FALSE as &Gate);
    (
      if n&8!=0 {gt} else {gf},
      if n&4!=0 {gt} else {gf},
      if n&2!=0 {gt} else {gf},
      if n&1!=0 {gt} else {gf}
    )
  }
  fn from_result<'a>((c,(n4,n3,n2,n1)): (&'a Gate,Nibble<'a>)) -> u8 {
    let mut n = 0u8;
    if c.get()  { n += 16};
    if n4.get() { n += 8 };
    if n3.get() { n += 4 };
    if n2.get() { n += 2 };
    if n1.get() { n += 1 };
    n
  }
  let ref ar = Arena::new();
  let gTrue = &TRUE as &Gate;
  let gFalse = &FALSE as &Gate;
  for n in range(0,std::u8::MAX) {
    let (nib1,nib2) = (n >> 4,n & 15);
    assert_eq!( nib1+nib2, from_result(four_bit_adder(ar,to_nib(nib1),to_nib(nib2),gFalse)) );
    assert_eq!(nib1+nib2+1,from_result(four_bit_adder(ar,to_nib(nib1),to_nib(nib2),gTrue )) );
  }
}
