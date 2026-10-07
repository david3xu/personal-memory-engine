// Launch the desktop shell around the local memory runtime.
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("desktop runtime failed");
}
