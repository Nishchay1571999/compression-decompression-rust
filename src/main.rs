// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::env::args;
// use std::fs::File;
// use std::io::copy;
// use std::io::BufReader;
// use std::time::Instant;
use std::fs;
use std::io;
use zip;


fn main(){

    // if args().len() != 3 {
    //     eprintln!("Usage: 'source' 'target' ");
    //     return;
    // }

    // let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // let output = File::create(args().nth(2).unwrap()).unwrap();
    // let mut encoder = GzEncoder::new(output, Compression::default());
    // let start = Instant::now();
    // copy(&mut input, &mut encoder).unwrap();
    // let output = encoder.finish().unwrap();
    // println!("Source Len: {:?}", input.get_ref().metadata().unwrap().len());
    // println!("Target Len: {:?}", output.metadata().unwrap().len());
    // println!("Elapsed Time: {:?}",start.elapsed());
    std::process::exit(decompress());
}


fn decompress() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>",args[0]);
        return 0; 
    }
    let fname = std::path::Path::new(&*&args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive =  zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {

        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if comment.is_empty() {
                println!("File : {} comment: {}",i,comment);
            }
        }
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\" ",i,outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        }else {
            println!("File {} extracted to \"{}\" ({} bytes) ",i,outpath.display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap()
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::prelude::PermissionsExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }

        }
    }
    return 0;
}