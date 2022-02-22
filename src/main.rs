mod closures;
mod data_types;
mod defining_enum;
mod defining_modules;
mod errors_with_panic;
mod errors_with_result;
mod example_collections;
mod example_struct;
mod flow_control;
mod functions;
mod generics_example;
mod guess_the_number;
mod hash_maps;
mod iterators;
mod lifetime_syntax;
mod logger;
mod match_flow_control;
mod mutate;
mod ownership;
mod panic_or_not_panic;
mod references_and_borrowing;
mod slices;
mod strings;
mod use_struct;
mod using_defined_modules;

extern crate adder;

fn main() {
    logger::info(&format!("Hello, {}!", "world"));
    guess_the_number::guess_the_number();
    mutate::mutate();
    data_types::data_types();
    functions::all();
    flow_control::flow_control();
    ownership::ownership();
    references_and_borrowing::references_and_borrowing();
    slices::slices();
    use_struct::use_struct();
    example_struct::example_struct();
    defining_enum::defining_enum();
    match_flow_control::match_flow_control();
    defining_modules::defining_modules();
    using_defined_modules::using_defined_modules();
    example_collections::example_collections();
    strings::strings();
    hash_maps::hash_maps();
    errors_with_panic::errors_with_panic();
    errors_with_result::errors_with_result();
    panic_or_not_panic::panic_or_not_panic();
    generics_example::generics_example();
    lifetime_syntax::lifetime_syntax();
    closures::closures();
    iterators::iterators();
    logger::info(&format!("adder: {}", adder::add_three(4))); // Given "adder: 7"
    logger::info("Good bye!");
}
