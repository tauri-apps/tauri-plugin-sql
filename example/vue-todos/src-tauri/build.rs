fn main() {
  println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.13");
  tauri_build::build()
}
