#![no_std]
use num_integer::Roots;

pub fn pair<I: Roots + Clone>(n: I, m: I) -> I{
    if n >= m{
        return n.clone() * n.clone() + n + m;
    }
    return m.clone() * m + n;
}
pub fn unpair<I: Roots + Clone>(z: I) -> (I,I){
    let q = z.sqrt();
    let l = z - q.clone() * q.clone();
    if l < q{
        return (l,q);
    }
    return (q.clone(),l - q);
}