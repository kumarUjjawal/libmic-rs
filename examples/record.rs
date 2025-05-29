use libmic_rs::mic::Recorder;

fn main() {
    Recorder::record_to_file("output.wav", 5).expect("Recording Failed");
}
