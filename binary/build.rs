use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    std::process::Command::new("trunk")
        .current_dir("../frontend")
        .args(["build", "--release"])
        .spawn()?;
    resource_dir("../frontend/dist").build()
}
