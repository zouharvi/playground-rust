use std::fs::File;
use std::io::prelude::*;
use tch::Kind;
use tch::Tensor;

// https://www.kaggle.com/competitions/titanic/

fn main() {
    let data_train = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/titanic/train.csv")
        .unwrap()
        .into_records()
        .map(|r| {
            let r = &r.unwrap();
            (
                r[1].parse::<i64>().unwrap(), // survived
                vec![
                    // r[0].parse::<f32>().unwrap(), // id
                    r[2].parse::<f32>().unwrap(),            // class
                    if &r[4] == "male" { 0.0 } else { 1.0 }, // sex
                    r[5].parse::<f32>().unwrap_or(20.0),     // age (missing)
                    r[6].parse::<f32>().unwrap(),            // SibSb (?)
                    r[7].parse::<f32>().unwrap(),            // Parch (?)
                    r[9].parse::<f32>().unwrap(),            // fare
                                                             // match &r[10] { "C"=>  0.0, "Q" => 1.0, "S" => 2.0},   // embarked
                ],
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
        .from_path("data/titanic/test.csv")
        .unwrap()
        .into_records()
        .map(|r| {
            let r = &r.unwrap();
            (
                r[0].parse::<i64>().unwrap(), // id
                vec![
                    r[1].parse::<f32>().unwrap(),            // class
                    if &r[3] == "male" { 0.0 } else { 1.0 }, // sex
                    r[4].parse::<f32>().unwrap_or(20.0),     // age (missing)
                    r[5].parse::<f32>().unwrap(),            // SibSb (?)
                    r[6].parse::<f32>().unwrap(),            // Parch (?)
                    r[8].parse::<f32>().unwrap_or(0.0),      // fare
                                                             // match &r[10] { "C"=>  0.0, "Q" => 1.0, "S" => 2.0},   // embarked
                ],
            )
        })
        .collect::<Vec<(i64, Vec<f32>)>>();

    let data_test_x: Tensor = Tensor::vstack(
        &data_test
            .iter()
            .map(|x| Tensor::of_slice::<f32>(&x.1))
            .collect::<Vec<Tensor>>(),
    );
    let data_test_id = &data_test.iter().map(|x| x.0).collect::<Vec<i64>>();

    let output = run((data_train_x, data_train_y), data_test_x).unwrap();

    let mut file = File::create("data/titanic/test_pred.csv").unwrap();
    file.write_all(b"PassengerId,Survived\n").unwrap();
    for (id, pred) in data_test_id.iter().zip(output.iter()) {
        let line = format!("{},{}\n", id, if *pred { 1 } else { 0 });
        file.write_all(line.as_bytes()).unwrap();
    }
}

use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs / "layer1", 6, 64, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs / "layer2", 64, 32, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs / "layer3", 32, 2, Default::default()))
        .add_fn(|xs| xs.softmax(1, Kind::Float))
    // .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
}

pub fn run(data_train: (Tensor, Tensor), data_test: Tensor) -> Result<Vec<bool>> {
    let vs = nn::VarStore::new(Device::Cpu);
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;

    for epoch in 1..500 {
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
