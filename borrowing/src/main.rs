use wavegen::{wf, sine};
use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;

fn main() {
    let waveform = wf!(f32, 200., sine!(50., 10.));
    let measurment: Vec<f32> = waveform.iter().take(100).collect();
    

    let (data, _spectrum) = fast_fourier_transform(measurment.clone());

    println!("{:?}", measurment);
    println!("length of measurement = {}", measurment.len());
    println!("length of data = {}", data.len());
    println!("{:?}", data);
}

fn fast_fourier_transform(mut data: Vec<f32>) -> (Vec<f32>, Vec<Complex<f32>>){
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(100);

    // make a vector for storing the spectrum
    let mut spectrum = fft.make_output_vec();

    fft.process(&mut data, &mut spectrum).unwrap();

    (data, spectrum)
}