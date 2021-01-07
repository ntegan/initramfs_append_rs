use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use std::io::Read;
use std::io::Write;

fn get_unzipped_data(file: &File) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decoder = GzDecoder::new(file);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;

    Ok(decoded_data)

}


fn append_file_to_initramfs(fill: &mut File, initramfs: &mut File) -> Result<(), Box<dyn std::error::Error>>{
    let initramfs_data_in = get_unzipped_data(&initramfs)?;
    let mut file_data = Vec::new();
    fill.read_to_end(&mut file_data)?;

    let cpio_archive = Vec::new();

    let builder = cpio::newc::Builder::new("appended");
    let mut writer = builder.write(cpio_archive, file_data.len() as u32);

    let mut data: &[u8] = &file_data.clone();
    std::io::copy(&mut data, &mut writer)?;
    let output = writer.finish()?;
    let output = cpio::newc::trailer(output)?;

    // TODO: can't figure out how to use Creekmore's cpio crate
    // https://docs.rs/crate/cpio/0.2.0/source/src/newc.rs
    // could just use a command lol

    let mut output2: Vec<u8> = Vec::new();
    for byte in output {
        output2.push(byte);
    }
    for byte in initramfs_data_in {
        output2.push(byte);
    }

    let mut output_file = File::create(Path::new("a.out"))?;
    output_file.write_all(&output2)?;

    panic!("okkk");
}

const INITRAMFS_FILE:&'static str = "./initramfs";
const INITRAMFS_OUTFILE:&'static str = "./initramfs.out";
const APPEND_FILE:&'static str = "./append_me.txt";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut initramfs_in = File::open(Path::new(INITRAMFS_FILE))?;
    let mut append_file = File::open(Path::new(APPEND_FILE))?;

    append_file_to_initramfs(&mut append_file, &mut initramfs_in)?;

    Ok(())
}
