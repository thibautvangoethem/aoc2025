use std::io::Write;
use std::{fs, path::Path};

pub fn generate(path_code: &str, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let day_folder = format!("day{:0>2}", day);
    let full_path = Path::new(path_code).join(day_folder);

    if let Err(e) = fs::create_dir(&full_path) {
        eprintln!("Error creating directory: {}", e);
        return Err(e.into());
    }
    println!("Created directory: {}", full_path.display());

    let inner_mod_file_path = full_path.join("mod.rs");
    if let Err(e) = fs::File::create(&inner_mod_file_path) {
        eprintln!("Error creating mod.rs file: {}", e);
        return Err(e.into());
    }
    let modcontent = format!("pub mod day{:0>2};", day);
    if let Err(e) = fs::write(&inner_mod_file_path, modcontent) {
        eprintln!("Error writing to mod file: {}", e);
        return Err(e.into());
    }
    println!("Created mod.rs file: {}", inner_mod_file_path.display());

    let inner_day_file_path = full_path.join(format!("day{:0>2}.rs", day));
    if let Err(e) = fs::File::create(&inner_day_file_path) {
        eprintln!("Error creating day file: {}", e);
        return Err(e.into());
    }
    println!("Created day file: {}", inner_day_file_path.display());
    let daycontent = "
    pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}";
    if let Err(e) = fs::write(&inner_day_file_path, daycontent) {
        eprintln!("Error writing to day file: {}", e);
        return Err(e.into());
    }
    println!(
        "Added content to day file: {}",
        inner_day_file_path.display()
    );

    let mod_file_path = Path::new(path_code).join("mod.rs");
    let mut file = match fs::File::options()
        .append(true)
        .create(true)
        .open(mod_file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening mod.rs file: {}", e);
            return Err(Box::new(e));
        }
    };

    if let Err(e) = writeln!(&mut file, "pub mod day{:0>2};", day) {
        eprintln!("Error writing to mod.rs file: {}", e);
        return Err(e.into());
    }

    Ok(())
}
