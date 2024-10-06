fn main() {
    println!("cargo:rustc-link-arg=/export:VerQueryValueW=hijacked.VerQueryValueW,@16");
    println!("cargo:rustc-link-arg=/export:GetFileVersionInfoSizeW=hijacked.GetFileVersionInfoSizeW,@7");
    println!("cargo:rustc-link-arg=/export:GetFileVersionInfoW=hijacked.GetFileVersionInfoW,@8");
    println!("cargo:rustc-link-arg=/export:GetFileVersionInfoA=hijacked.GetFileVersionInfoA,@1");
    println!("cargo:rustc-link-arg=/export:VerQueryValueA=hijacked.VerQueryValueA,@22");
    println!("cargo:rustc-link-arg=/export:GetFileVersionInfoSizeA=hijacked.GetFileVersionInfoSizeA,@5");
}