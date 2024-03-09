use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub fn init_got_dir(path: Option<&Path>) -> std::io::Result<()> {
    let got_dir = match path {
        Some(path) => path,
        None => Path::new(".got"),
    };

    create_dir(got_dir)?;
    init_objects_dir(got_dir)?;
    init_refs_dir(got_dir)?;
    init_hooks_dir(got_dir)?;
    write_to_file(&got_dir.join("HEAD"), "ref: refs/heads/master")?;
    write_to_file(&got_dir.join("config"), "")?;
    Ok(())
}

fn create_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

fn write_to_file(path: &Path, contents: &str) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

fn append_to_file(path: &Path, contents: &str) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
    file.write_all(contents.as_bytes())
}

fn init_refs_dir(got_dir: &Path) -> std::io::Result<()> {
    let refs_dir: PathBuf = got_dir.join("refs");
    create_dir(&refs_dir)?;
    write_to_file(&refs_dir.join("heads"), "")?;
    write_to_file(&refs_dir.join("tags"), "")?;

    Ok(())
}

fn init_objects_dir(got_dir: &Path) -> std::io::Result<()> {
    create_dir(&got_dir.join("objects"))?;

    Ok(())
}

fn init_hooks_dir(got_dir: &Path) -> std::io::Result<()> {
    let hooks_dir: PathBuf = got_dir.join("hooks");
    create_dir(&hooks_dir)?;
    write_to_file(&hooks_dir.join("prepare-commit-msg"), "")?;

    Ok(())
}
