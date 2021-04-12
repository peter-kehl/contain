#![cfg(test)]

#[test]
fn it_works() {
}

struct Pat(i32);
enum PatEnum {
    Pat(Pat)
}

#[test]
fn pattern_replacement() {
    let super::pattern!(inner) = Pat(1);
    println!("Inner: {}", inner);

    let pat_enum = PatEnum::Pat(Pat(5));
    let PatEnum::Pat(super::pattern!(inner)) = pat_enum;
    println!("Inner: {}", inner);

    match pat_enum {
        PatEnum::Pat(super::pattern!(inner)) => {
            println!("Inner: {}", inner);
        }
    }
}

fn accept_pat(Pat(value): Pat) {}
fn accept_pat_by_macro(super::pattern!{value}: Pat) {}
//fn f((i-2):i32) {}

#[test]
fn fn_parameter_as_pattern_replacement() {
    
}