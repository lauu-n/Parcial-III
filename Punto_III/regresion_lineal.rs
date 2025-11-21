use std::time::Instant;
use sysinfo::{System, Pid, PidExt};
use plotters::prelude::*;
use std::f64;

fn linear_regression_rust(
    x: &[f64], 
    y: &[f64], 
    learning_rate: f64, 
    epochs: usize
) -> (
    f64, f64, f64, 
    Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>
) {
    let mut w = 0.0;
    let mut b = 0.0;
    let m = x.len() as f64;
    let start_time = Instant::now();
    let mut sys = System::new();
    
    // Vectores para métricas
    let mut losses = Vec::new();
    let mut weights = Vec::new();
    let mut biases = Vec::new();
    let mut memory_usage = Vec::new();
    let mut timestamps = Vec::new();
    
    for epoch in 0..epochs {
        // Actualizar información del sistema
        sys.refresh_all();
        
        // Cálculo de predicciones y error
        let y_pred: Vec<f64> = x.iter().map(|&xi| w * xi + b).collect();
        let error: Vec<f64> = y_pred.iter().zip(y.iter()).map(|(&yp, &yi)| yp - yi).collect();
        
        // Cálculo de gradientes
        let dw: f64 = (2.0 / m) * error.iter().zip(x.iter()).map(|(&e, &xi)| e * xi).sum::<f64>();
        let db: f64 = (2.0 / m) * error.iter().sum::<f64>();
        
        // Actualización de parámetros
        w -= learning_rate * dw;
        b -= learning_rate * db;
        
        // Calcular métricas
        let mse: f64 = error.iter().map(|&e| e * e).sum::<f64>() / m;
        let current_time = start_time.elapsed().as_secs_f64();
        
        // Obtener uso de memoria del proceso actual (manejo seguro)
        let current_memory_kb = if let Some(pid) = sysinfo::get_current_pid() {
            if let Some(process) = sys.process(pid) {
                process.memory() as f64 / 1024.0 // Convertir a KB
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Guardar métricas cada 50 epochs
        if epoch % 50 == 0 {
            losses.push(mse);
            weights.push(w);
            biases.push(b);
            memory_usage.push(current_memory_kb);
            timestamps.push(current_time);
        }
        
        if (epoch + 1) % 200 == 0 {
            println!("Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}", 
                    epoch + 1, mse, w, b);
        }
    }
    
    let execution_time = start_time.elapsed().as_secs_f64();
    
    (w, b, execution_time, losses, weights, biases, memory_usage, timestamps)
}

fn plot_rust_metrics(
    losses: &[f64],
    weights: &[f64],
    biases: &[f64],
    memory_usage: &[f64],
    timestamps: &[f64]
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("rust_metrics.png", (1200, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let root = root.titled("Métricas de Regresión Lineal - RUST", ("sans-serif", 20))?;
    
    let charts = RootElement::new(root).split_evenly((2, 2));
    
    // Gráfica 1: Evolución del Error
    let max_epoch = losses.len() as f64 * 50.0;
    let max_loss = losses.iter().cloned().fold(f64::NAN, f64::max) * 1.1;
    
    let mut chart1 = ChartBuilder::on(&charts[0])
        .caption("Evolución del Error (MSE)", ("sans-serif", 16))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..max_epoch, 0f64..max_loss)?;
    
    chart1.configure_mesh().draw()?;
    
    let epochs_plot: Vec<f64> = (0..losses.len()).map(|i| i as f64 * 50.0).collect();
    chart1.draw_series(LineSeries::new(
        epochs_plot.iter().zip(losses.iter()).map(|(&x, &y)| (x, *y)),
        &BLUE,
    ))?.label("MSE")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
    
    // Gráfica 2: Evolución de Parámetros
    let max_weight = weights.iter().cloned().fold(f64::NAN, f64::max);
    let min_bias = biases.iter().cloned().fold(f64::NAN, f64::min);
    let y_range = (min_bias - 0.5).max(-1.0)..(max_weight + 0.5).min(3.0);
    
    let mut chart2 = ChartBuilder::on(&charts[1])
        .caption("Evolución de Parámetros", ("sans-serif", 16))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..max_epoch, y_range)?;
    
    chart2.configure_mesh().draw()?;
    
    chart2.draw_series(LineSeries::new(
        epochs_plot.iter().zip(weights.iter()).map(|(&x, &y)| (x, *y)),
        &RED,
    ))?.label("w")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    
    chart2.draw_series(LineSeries::new(
        epochs_plot.iter().zip(biases.iter()).map(|(&x, &y)| (x, *y)),
        &GREEN,
    ))?.label("b")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));
    
    chart2.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;
    
    // Gráfica 3: Uso de Memoria
    let max_memory = memory_usage.iter().cloned().fold(f64::NAN, f64::max) * 1.1;
    let max_time = timestamps[timestamps.len()-1];
    
    let mut chart3 = ChartBuilder::on(&charts[2])
        .caption("Uso de Memoria", ("sans-serif", 16))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..max_time, 0f64..max_memory)?;
    
    chart3.configure_mesh().draw()?;
    
    chart3.draw_series(LineSeries::new(
        timestamps.iter().zip(memory_usage.iter()).map(|(&x, &y)| (x, *y)),
        &MAGENTA,
    ))?;
    
    // Gráfica 4: Convergencia
    let max_w = weights.iter().cloned().fold(f64::NAN, f64::max);
    
    let mut chart4 = ChartBuilder::on(&charts[3])
        .caption("Convergencia", ("sans-serif", 16))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..max_w, 0f64..max_loss)?;
    
    chart4.configure_mesh().draw()?;
    
    chart4.draw_series(LineSeries::new(
        weights.iter().zip(losses.iter()).map(|(&x, &y)| (x, *y)),
        &ORANGE,
    ))?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== EJECUCIÓN EN RUST ===");
    
    // Datos de ejemplo
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    
    let learning_rate = 0.01;
    let epochs = 1000;
    
    let (w, b, execution_time, losses, weights, biases, memory_usage, timestamps) = 
        linear_regression_rust(&x, &y, learning_rate, epochs);
    
    println!("\nResultados RUST:");
    println!("w ≈ {:.6}, b ≈ {:.6}", w, b);
    println!("Tiempo total: {:.6} segundos", execution_time);
    
    // Generar gráficas
    plot_rust_metrics(&losses, &weights, &biases, &memory_usage, &timestamps)?;
    println!("Gráficas guardadas como 'rust_metrics.png'");
    
    // Prueba del modelo
    let x_nuevo = 7.0;
    let y_pred_nuevo = w * x_nuevo + b;
    println!("Para x = {}, y_pred ≈ {:.4}", x_nuevo, y_pred_nuevo);
    
    Ok(())
}