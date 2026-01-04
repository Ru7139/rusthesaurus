use screencapturekit::prelude::*;

struct Handler;

impl SCStreamOutputTrait for Handler {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, _type: SCStreamOutputType) {
        println!("Received frame at {:?}", sample.presentation_timestamp());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get available displays
    let content = SCShareableContent::get()?;
    let display = &content.displays()[0];

    // Configure capture
    let filter = SCContentFilter::create()
        .with_display(display)
        .with_excluding_windows(&[])
        .build();

    let config = SCStreamConfiguration::new()
        .with_width(1920)
        .with_height(1080)
        .with_pixel_format(PixelFormat::BGRA);

    // Start streaming
    let mut stream = SCStream::new(&filter, &config);
    stream.add_output_handler(Handler, SCStreamOutputType::Screen);
    stream.start_capture()?;

    // Capture runs in background...
    std::thread::sleep(std::time::Duration::from_secs(5));

    stream.stop_capture()?;

    Ok(())
}
