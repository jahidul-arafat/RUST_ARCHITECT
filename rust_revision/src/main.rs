// mod basics;
// mod challenge01;
// mod compound_datatypes;
// mod function_sim;
// mod challenge02;
// mod program_flow_control;
// mod challenge03;
// mod ownership_sim;
// mod references_sim;
mod data_race_sim;
// mod str_slice;
// mod challenge04;
// mod module_sim;
// mod challenge05;
// mod cli_io_sim;
// mod file_io_sim;
// mod challenge06;
// mod challenge06_cli;
//mod struct_sim;
//mod tuple_struct_sim;
//mod challenge07;
//mod generic_type_sim;
//mod generic_type_method_sim;
//mod generic_box_datatype_sim;
// mod challenge08;
// mod playground;
// mod traits_sim;
// mod traits_bound_sim;
mod challenge09;
mod lifetime_sim;
mod lifetime_elision_rules;
mod lifetime_handling_vulnerabilities;
mod lifetime_struct;
mod data_race_simplified;
mod lifetime_static;
//mod suppliment_user_input;

fn main() {
    println!("RUST MAIN");
    //basics::simulation();
    //challenge01::challenge();
    //compound_datatypes::simulation();
    //function_sim::simulation();
    //challenge02::challenge();
    //program_flow_control::simulation();
    //challenge03::challenge();
    //ownership_sim::simulation();
    //references_sim::simulation();
    //data_race_sim::simulation();
    //data_race_simplified::simulation();
    //str_slice::simulation();
    //challenge04::challenge();
    //module_sim::simulation();
    //challenge05::challenge();
    //suppliment_user_input::supplementary();
    //cli_io_sim::simulation();
    //file_io_sim::simulation();
    //challenge06::challenge();
    //challenge06_cli::challenge();
    //struct_sim::simulation();
    //tuple_struct_sim::simulation();
    //challenge07::challenge();
    //generic_type_sim::simulation();
    //generic_type_method_sim::simulation();
    //generic_box_datatype_sim::simulation();
    //challenge08::challenge();
    //playground::playground();
    //traits_sim::simulation();
    //traits_bound_sim::simulation();
    //challenge09::challenge();
    //lifetime_sim::simulation();
    //lifetime_elision_rules::simulation();
    //lifetime_handling_vulnerabilities::simulation();
    //lifetime_struct::simulation();
    lifetime_static::simulation();
}
