use std::fs::File;
use std::io::prelude::*;
use csv::StringRecord;
use tch::Kind;
use tch::Tensor;

// https://www.kaggle.com/competitions/spaceship-titanic/

// PassengerId,HomePlanet,CryoSleep,Cabin,Destination,Age,VIP,RoomService,FoodCourt,ShoppingMall,Spa,VRDeck,Name,Transported

fn load_row(r: &StringRecord) -> Vec<f32> {
    vec![
        // r[0].parse::<f32>().unwrap(), // id
        // r[1] home planet
        ((&r[1] == "Earth") as i32) as f32,
        ((&r[1] == "Mars") as i32) as f32,
        ((&r[1] == "Europa") as i32) as f32,
        if &r[2] == "False" { 0.0 } else { 1.0 }, // cryo sleep
        // r[3] cabin
        // r[4] destination
        ((&r[4] == "TRAPPIST-1e") as i32) as f32,
        ((&r[4] == "55 Cancri e") as i32) as f32,
        ((&r[4] == "PSO J318.5-22") as i32) as f32,
        // age
        r[5].parse::<f32>().unwrap_or(30.0),
        // VIP
        if &r[6] == "False" { 0.0 } else { 1.0 },
        // Room service
        r[7].parse::<f32>().unwrap_or(0.0),
        // Food court
        r[8].parse::<f32>().unwrap_or(0.0),
        // Shopping mall
        r[9].parse::<f32>().unwrap_or(0.0),
        // Spa
        r[10].parse::<f32>().unwrap_or(0.0),
        // VR deck
        r[11].parse::<f32>().unwrap_or(0.0),
        // match &r[10] { "C"=>  0.0, "Q" => 1.0, "S" => 2.0},   // embarked
    ].to_vec()
}

fn main() {
    let data_train = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/train.csv")
        .unwrap()
        .into_records()
        .map(|r| {
            let r = &r.unwrap();
            (
                if &r[13] == "False" { 0 } else { 1 },
                load_row(r)
            )
        })
        .collect::<Vec<(i64, Vec<f32>)>>();

    let data_train_y: Tensor =
        Tensor::of_slice::<i64>(&data_train.iter().map(|x| x.0).collect::<Vec<i64>>());
    let data_train_x: Tensor = Tensor::vstack(
        &data_train
            .iter()
            .map(|x| Tensor::of_slice::<f32>(&x.1))
            .collect::<Vec<Tensor>>(),
    );

    println!("Train: {:?} {:?}", data_train_x.size(), data_train_y.size());

    let data_test = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/test.csv")
        .unwrap()
        .into_records()
        .map(|r| {
            let r = &r.unwrap();
            (
                // id
                r[0].to_string(),
                load_row(r)
            )
        })
        .collect::<Vec<(String, Vec<f32>)>>();

    let data_test_x: Tensor = Tensor::vstack(
        &data_test
            .iter()
            .map(|x| Tensor::of_slice::<f32>(&x.1))
            .collect::<Vec<Tensor>>(),
    );
    let data_test_id = data_test.into_iter().map(|x| x.0).collect::<Vec<String>>();

    let output = run((data_train_x, data_train_y), data_test_x).unwrap();

    let mut file = File::create("data/test_pred.csv").unwrap();
    file.write_all(b"PassengerId,Transported\n").unwrap();
    for (id, pred) in data_test_id.iter().zip(output.iter()) {
        let line = format!("{},{}\n", id, if *pred { "True" } else { "False" });
        file.write_all(line.as_bytes()).unwrap();
    }
}

use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs / "layer1", 14, 32, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs / "layer2", 32, 32, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs / "layer3", 32, 2, Default::default()))
        .add_fn(|xs| xs.softmax(1, Kind::Float))
    // .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
}

pub fn run(data_train: (Tensor, Tensor), data_test: Tensor) -> Result<Vec<bool>> {
    let vs = nn::VarStore::new(Device::Cpu);
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 5e-3)?;

    for epoch in 1..10 {
        let loss = net
            .forward(&data_train.0)
            .cross_entropy_for_logits(&data_train.1);
        opt.backward_step(&loss);

        let test_accuracy = net
            .forward(&data_train.0)
            .accuracy_for_logits(&data_train.1);
        println!(
            "epoch: {:4} train loss: {:8.5} train acc: {:5.2}%",
            epoch,
            f64::from(&loss),
            100. * f64::from(&test_accuracy),
        );
    }

    let output = net.forward(&data_test);
    let output: Vec<bool> = Vec::<Vec<f32>>::from(output)
        .iter()
        .map(|x| x[0] < x[1])
        .collect();
    Ok(output)
}
