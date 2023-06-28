use log::info;
use tch::nn::{Module, OptimizerConfig};
use crate::data::fetcher::fetch;
use crate::data::mapper::convert_data;
use crate::util::time::TimeUnit;

pub async fn test_nn() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let pure_data = fetch("AAPL", &TimeUnit::Years(10)).await?;
    let _mapped_data = convert_data(&pure_data)?;
    info!("Fetched and mapped data!");

    // Create the neural network with tch.
    let vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let net = tch::nn::seq()
        .add(tch::nn::linear(vs.root(), 1, 1, Default::default()));
    info!("Created neural network!");

    // Create the optimizer.
    let mut opt = tch::nn::Adam::default().build(&vs, 1e-3)?;
    info!("Created optimizer!");

    // Train the neural network.
    for epoch in 1..1001 {
        info!("Epoch: {}", epoch);
        let data = pure_data.clone();
        let x = data.iter().map(|x| x.0).collect::<Vec<f64>>();
        let y = data.iter().map(|x| x.1).collect::<Vec<f64>>();

        info!("Converting data...");
        // Convert the target values to the appropriate data type
        let y = y.into_iter().collect::<Vec<f64>>();
        let y = tch::Tensor::from_slice(&y).reshape([y.len() as i64, 1]);
        let x = x.into_iter().collect::<Vec<f64>>();

        info!("Calculating loss...");
        let output = net.forward(&tch::Tensor::from_slice(&x).to_kind(tch::Kind::Float));
        let output = output.squeeze(); // Remove extra dimension
        let output = output.view([output.size()[0], 1]); // Reshape to [batch_size, 1]
        let loss = output.mse_loss(&y, tch::Reduction::Mean);
        info!("Calculated loss!");

        info!("Backward step...");
        opt.backward_step(&loss);
        info!("Backward step done!");

        if epoch % 100 == 0 {
            let loss = loss.double_value(&[]);
            println!("epoch: {:4} train loss: {}", epoch, loss);
        }
    }

    Ok(())
}
