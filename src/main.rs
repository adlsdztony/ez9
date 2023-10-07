use image::GenericImageView;
use std::env;
use std::path::Path;
// regist right click menu
use winreg::enums::*;

type PassError = Box<dyn std::error::Error>;


fn cut_image(img_path: &str) {
    let img = image::open(img_path).unwrap();

    let (width, height) = img.dimensions();
    let length = if width > height { height } else { width };

    let img = img.resize(length, length, image::imageops::FilterType::Nearest);

    let cut = 8;

    let single_length = (length - cut*2) / 3;

    let img_up_left = img.crop_imm(0, 0, single_length, single_length);
    let img_center_left = img.crop_imm(0, single_length + cut, single_length, single_length);
    let img_down_left = img.crop_imm(0, single_length*2 + cut*2, single_length, single_length);

    let img_up_center = img.crop_imm(single_length + cut, 0, single_length, single_length);
    let img_center_center = img.crop_imm(single_length + cut, single_length + cut, single_length, single_length);
    let img_down_center = img.crop_imm(single_length + cut, single_length*2 + cut*2, single_length, single_length);

    let img_up_right = img.crop_imm(single_length*2 + cut*2, 0, single_length, single_length);
    let img_center_right = img.crop_imm(single_length*2 + cut*2, single_length + cut, single_length, single_length);
    let img_down_right = img.crop_imm(single_length*2 + cut*2, single_length*2 + cut*2, single_length, single_length);

    let img_name = Path::new(img_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    img_up_left.save(format!("{}_1.jpg", img_name)).unwrap();
    img_center_left.save(format!("{}_4.jpg", img_name)).unwrap();
    img_down_left.save(format!("{}_7.jpg", img_name)).unwrap();

    img_up_center.save(format!("{}_2.jpg", img_name)).unwrap();
    img_center_center.save(format!("{}_5.jpg", img_name)).unwrap();
    img_down_center.save(format!("{}_8.jpg", img_name)).unwrap();

    img_up_right.save(format!("{}_3.jpg", img_name)).unwrap();
    img_center_right.save(format!("{}_6.jpg", img_name)).unwrap();
    img_down_right.save(format!("{}_9.jpg", img_name)).unwrap();

}

fn add_regist() -> Result<(), PassError> {
    let hkcr = winreg::RegKey::predef(HKEY_CLASSES_ROOT);
    // right click menu on image
    let shell = hkcr.open_subkey_with_flags("SystemFileAssociations\\image\\shell", KEY_WRITE)?;
    let (app, _) = shell.create_subkey("cut_image")?;
    app.set_value("", &"cut image")?;
    let (command, _) = app.create_subkey("command")?;

    // get current path
    let current_path = env::current_exe()?;
    let current_path = current_path.to_str().unwrap_or_else(|| {
        panic!("current path is not a valid UTF-8 sequence");
    });

    // set command
    command.set_value("", &format!("{} %1", current_path))?;
    println!("{}", current_path);
    println!("add right click menu success!");
    // pause here
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}

fn delete_regist() -> Result<(), PassError> {
    let hkcr = winreg::RegKey::predef(HKEY_CLASSES_ROOT);
    // right click menu on image
    let shell = hkcr.open_subkey_with_flags("SystemFileAssociations\\image\\shell", KEY_WRITE)?;
    let app = shell.open_subkey("cut_image")?;

    app.delete_subkey("command")?;
    shell.delete_subkey("cut_image")?;

    println!("delete right click menu success!");
    // pause here
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}

fn print_help() {
    println!("Usage: ez9 [image1_path] [image2_path] ...");
    println!("add or delete rigest: ez9");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // ask user whether to addor delete right click menu
        println!("Please run as administrator");
        print_help();
        println!("\nDo you want to add or delete right click menu? [y/n]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            return;
        }

        // whether there is a right click menu
        let hkcr = winreg::RegKey::predef(HKEY_CLASSES_ROOT);
        let shell = hkcr.open_subkey("SystemFileAssociations\\image\\shell").unwrap_or_else(|err| {
            panic!("Open shell failed: {}\nPlease try to run as administrator\n", err);
        });
        let cut_image = shell.open_subkey("cut_image");
        if cut_image.is_ok() {
            // ask user whether to delete right click menu
            println!("\nThere is a right click menu");
            println!("Do you want to delete right click menu? [y/n]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                return;
            }

            delete_regist().unwrap_or_else(|err| {
                println!("delete right click menu failed: {}\nPlease try to run as administrator\n", err);
            });
            return;
        }

        // add right click menu
        add_regist().unwrap_or_else(|err| {
            println!("add right click menu failed: {}\nPlease try to run as administrator\n", err);
        });
        return;
    }

    // cut images
    for img_path in args.iter().skip(1) {
        cut_image(img_path);
    }
}
