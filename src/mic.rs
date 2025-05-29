use crate::error::MicError;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct Recorder;

impl Recorder {
    pub fn record_to_file(path: &str, duration_sec: u64) -> Result<(), MicError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(MicError::DeviceNotFound)?;
        let config = device
            .default_input_config()
            .map_err(|e| MicError::Other(format!("Failed to get input config: {}", e)))?;

        println!(
            "Recording with device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        );
        println!("Input format: {:?}", config);

        let sample_format = config.sample_format();
        let config = config.into();

        match sample_format {
            cpal::SampleFormat::F32 => Self::record::<f32>(&device, &config, path, duration_sec),
            cpal::SampleFormat::I16 => Self::record::<i16>(&device, &config, path, duration_sec),
            cpal::SampleFormat::I32 => Self::record::<i32>(&device, &config, path, duration_sec),
            cpal::SampleFormat::I8 => Self::record::<i8>(&device, &config, path, duration_sec),
            _ => {
                return Err(MicError::Other(format!(
                    "Unsupported format: {:?}",
                    sample_format
                )));
            }
        }
    }

    fn record<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        path: &str,
        duration_sec: u64,
    ) -> Result<(), MicError>
    where
        T: cpal::Sample + hound::Sample + Send + cpal::SizedSample + 'static,
    {
        let spec = hound::WavSpec {
            channels: config.channels,
            sample_rate: config.sample_rate.0,
            bits_per_sample: std::mem::size_of::<T>() as u16 * 8,
            sample_format: if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>() {
                hound::SampleFormat::Float
            } else {
                hound::SampleFormat::Int
            },
        };

        let writer = hound::WavWriter::create(path, spec)
            .map_err(|e| MicError::Other(format!("Failed to create WAV writer: {}", e)))?;
        let writer = Arc::new(Mutex::new(Some(writer)));
        let writer_clone = Arc::clone(&writer);

        let err_fn = |err| eprintln!("Stream error: {}", err);

        let stream = device
            .build_input_stream(
                config,
                move |data: &[T], _| {
                    let mut writer_lock = writer_clone.lock().unwrap();
                    if let Some(w) = writer_lock.as_mut() {
                        for &sample in data {
                            if let Err(e) = w.write_sample(sample) {
                                eprintln!("Error writing sample: {}", e);
                            }
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| MicError::Other(format!("Failed to build input stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| MicError::Other(format!("Failed to start stream: {}", e)))?;

        std::thread::sleep(std::time::Duration::from_secs(duration_sec));

        // Stop the stream and finalize the writer
        drop(stream);

        writer
            .lock()
            .unwrap()
            .take()
            .map(|w| w.finalize())
            .transpose()
            .map_err(|e| MicError::Other(format!("Failed to finalize WAV file: {}", e)))?;

        Ok(())
    }
}
