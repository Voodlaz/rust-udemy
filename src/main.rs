
mod part_2_types_and_variables;
mod part_3_control_flow;
use crate::part_2_types_and_varibles;
use crate::part_3_control_flow;
mod input;

fn main() {
    part_2_types_and_variables::core_datatypes::run();
    part_2_types_and_variables::operators::run();
    part_2_types_and_variables::constants::run();
    part_2_types_and_variables::scope::run();
    part_2_types_and_variables::stuck_and_heap::run();

    part_3_control_flow::if_statement::run();
    part_3_control_flow::loops::run();
    part_3_control_flow::match_statement::run();
    part_3_control_flow::combination_lock::run();
    // input::run();
}
