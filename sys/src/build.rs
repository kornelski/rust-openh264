extern crate pkg_config;

use std::path::Path;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io;

const VERSION: &'static str = "1.6.0";

fn main() {
    if pkg_config::probe_library("foo").is_err() {
        download_and_install();
    }
}

fn download_and_install() {
    let target = env::var_os("CARGO_TARGET_DIR").or(env::var_os("OUT_DIR")).unwrap();
    let target = Path::new(&target);

    let root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&root);
    let vendor = root.join("vendor");

    let bits = if cfg!(target_pointer_width="32") {"32"} else {"64"};
    let (lib_dest, link_name, url) = if cfg!(target_os="linux") {
        (target.join("libopenh264.3.so"), "openh264.3",
         format!("https://github.com/cisco/openh264/releases/download/v{}/libopenh264-{}-linux{}.3.so.bz2", VERSION, VERSION, bits))
    } else if cfg!(target_os="windows") {
        (target.join("openh264.dll"), "openh264",
         format!("https://github.com/cisco/openh264/releases/download/v{}/openh264-{}-win{}msvc.dll.bz2", VERSION, VERSION, bits))
    } else if cfg!(target_os="macos") {
        (Path::new("/usr/local/lib").join("libopenh264.3.dylib"), "openh264.3",
         format!("https://github.com/cisco/openh264/releases/download/v{}/libopenh264-{}-osx{}.3.dylib.bz2", VERSION, VERSION, bits))
    } else {
        panic!("unsupported platform");
    };

    let archive_filename = url.split("/").last().unwrap();
    let archive_dest = vendor.join(archive_filename);
    let lib_filename = &archive_filename[..archive_filename.len()-4];
    let lib_source = vendor.join(lib_filename);

    if missing(&lib_dest) || missing(&lib_source) {
        if missing(&lib_source) {
            println!("cargo:warning=Missing {}", lib_source.display());
            if missing(&archive_dest) {
                println!("cargo:warning=Downloading {}", &url);
                Command::new("curl").arg("-sSfL").arg("-o").arg(&archive_dest).arg(&url).status()
                     .expect(&format!("Please download {} to {}", url, archive_dest.display()));
            }
            Command::new("bunzip2").arg("--keep").arg("--force").arg("--decompress").arg(&archive_dest).status()
                 .expect(&format!("Please uncompress {} to {}", archive_dest.display(), lib_source.display()));
        }

        let mut src = File::open(&lib_source).expect("Archive didn't uncompress right");
        let mut dest = File::create(&lib_dest).unwrap();
        assert!(io::copy(&mut src, &mut dest).unwrap() > 0);
    }

    println!("cargo:rustc-link-lib={}", link_name);
    println!("cargo:rustc-link-search=native={}", lib_dest.parent().unwrap().display());
}

fn missing(path: &Path) -> bool {
    match path.metadata() {
        Ok(ref m) if m.len() > 1000 => false,
        _ => true,
    }
}
