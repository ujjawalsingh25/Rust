mod a__enum_basic;
mod b__match_pattern;

fn main() {

    println!("\n__________________  Enum Basic  __________________");
    a__enum_basic::enum_basic();      
    a__enum_basic::impl_in_enum();         
    a__enum_basic::option_enum();         
    
    println!("\n__________________  Match Control Flow Construct  __________________");
    b__match_pattern::match_pattern();
    println!("________  Matching Option<T>  ___________");
    b__match_pattern::matching_option();
    println!("________  Catch All Pattern  ___________");
    b__match_pattern::catch_all_pattern();
    println!("________  If Let Concise ControlFlow  ___________");
    b__match_pattern::if_let_concise_controlflow();      ;
}