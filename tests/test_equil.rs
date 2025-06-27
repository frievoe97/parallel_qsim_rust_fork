use std::path::PathBuf;

mod test_simulation;
use test_simulation::{execute_sim, execute_sim_with_channels, TestSubscriber, DummySubscriber};
use rust_q_sim::simulation::config::CommandLineArgs;
use rust_q_sim::simulation::id::store_to_file;
use rust_q_sim::simulation::messaging::communication::local_communicator::DummySimCommunicator;
use rust_q_sim::simulation::network::global_network::Network;
use rust_q_sim::simulation::population::population_data::Population;
use rust_q_sim::simulation::vehicles::garage::Garage;

fn create_resources(plan_file: &str, out_dir: &PathBuf) {
    let input_dir = PathBuf::from("./assets/equil/");
    let net = Network::from_file_as_is(&input_dir.join("equil-network.xml"));
    let mut garage = Garage::from_file(&input_dir.join("equil-vehicles.xml"));
    let pop = Population::from_file(&PathBuf::from(plan_file), &mut garage);

    store_to_file(&out_dir.join("ids.binpb"));
    net.to_file(&out_dir.join("equil-network.binpb"));
    pop.to_file(&out_dir.join("equil-plan.binpb"));
    garage.to_file(&out_dir.join("equil-vehicles.binpb"));
}

#[test]
fn execute_equil_single_part() {
    let test_dir = PathBuf::from("./test_output/simulation/equil_single_part/");
    create_resources("./assets/equil/equil-1-plan.xml", &test_dir);

    let config_args = CommandLineArgs {
        config_path: "./tests/resources/equil/equil-config-1.yml".to_string(),
        num_parts: None,
    };

    execute_sim(
        DummySimCommunicator(),
        Box::new(TestSubscriber::new_with_events_from_file(
            "./tests/resources/equil/expected_events.xml",
        )),
        config_args,
    );
}

#[test]
fn execute_equil_2_parts() {
    let test_dir = PathBuf::from("./test_output/simulation/equil_with_channels/");
    create_resources("./assets/equil/equil-1-plan.xml", &test_dir);

    let config_args = CommandLineArgs {
        config_path: "./tests/resources/equil/equil-config-2.yml".to_string(),
        num_parts: None,
    };

    execute_sim_with_channels(
        config_args,
        "./tests/resources/equil/expected_events.xml",
    );
}

#[test]
fn network_route_car_not_main_mode() {
    let test_dir = PathBuf::from("./test_output/simulation/equil_net_no_main/");
    create_resources("./assets/equil/equil-network-one-leg.xml", &test_dir);

    let config_args = CommandLineArgs {
        config_path: "./tests/resources/equil/equil-config-net-no-main.yml".to_string(),
        num_parts: None,
    };

    execute_sim(
        DummySimCommunicator(),
        Box::new(TestSubscriber::new_with_events_from_file(
            "./tests/resources/equil/expected_events_teleport.xml",
        )),
        config_args,
    );
}

#[test]
fn generic_route_car_not_main_mode() {
    let test_dir = PathBuf::from("./test_output/simulation/equil_gen_no_main/");
    create_resources("./assets/equil/equil-generic-one-leg.xml", &test_dir);

    let config_args = CommandLineArgs {
        config_path: "./tests/resources/equil/equil-config-gen-no-main.yml".to_string(),
        num_parts: None,
    };

    execute_sim(
        DummySimCommunicator(),
        Box::new(TestSubscriber::new_with_events_from_file(
            "./tests/resources/equil/expected_events_teleport.xml",
        )),
        config_args,
    );
}

#[test]
#[should_panic]
fn generic_route_car_main_mode_should_crash() {
    let test_dir = PathBuf::from("./test_output/simulation/equil_gen_main/");
    create_resources("./assets/equil/equil-generic-one-leg.xml", &test_dir);

    let config_args = CommandLineArgs {
        config_path: "./tests/resources/equil/equil-config-gen-main.yml".to_string(),
        num_parts: None,
    };

    execute_sim(
        DummySimCommunicator(),
        Box::new(DummySubscriber {}),
        config_args,
    );
}
