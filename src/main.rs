use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::os::windows::fs::FileExt;
use std::str;
use std::sync::Arc;
use crate::build_version::BuildVersion;
use crate::publisher::{Event, Publisher, Subscription};
use pdfium_render::prelude::*;

mod publisher;
mod build_version;

const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;
const PE_HEADER_OFFSET: u64 = 60;
const CHARACTERISTICS_OFFSET: u64 = 18;

fn main() {
    //Refer to https://github.com/pyinstaller/pyinstaller/issues/1288
    let mut file = File::
        options()
        .read(true)
        .write(true)
        .open(r"D:\test.exe", )
        .unwrap();
    let mut mz_header = [0u8;2];
    file.read(&mut mz_header).unwrap();
    if str::from_utf8(&mz_header).unwrap() != "MZ" {
        println!("Not MZ");
        return;
    }

    file.seek(SeekFrom::Start(PE_HEADER_OFFSET)).unwrap();
    let mut buffer = [0u8;4];
    file.read(&mut buffer).unwrap();
    let pe_header_location = u32::from_ne_bytes(buffer);
    file.seek(SeekFrom::Start(pe_header_location as u64)).unwrap();
    file.read(&mut buffer).unwrap();
    if str::from_utf8(&buffer).unwrap() != "PE\0\0" {
        println!("Error reading PE Header");
        return;
    }

    let characteristics_offset = pe_header_location as u64 + 4u64 + CHARACTERISTICS_OFFSET;
    file.seek(SeekFrom::Start(characteristics_offset)).unwrap();
    let mut bits = [0u8;2];
    file.read(&mut bits).unwrap();

    let large_address_aware = u16::from_ne_bytes(bits) & IMAGE_FILE_LARGE_ADDRESS_AWARE == IMAGE_FILE_LARGE_ADDRESS_AWARE;
    println!("LARGE ADDRESS AWARE: {}", large_address_aware);
    if large_address_aware {
        bits = (u16::from_ne_bytes(bits) & !IMAGE_FILE_LARGE_ADDRESS_AWARE).to_ne_bytes();
    }else{
        bits = (u16::from_ne_bytes(bits) | IMAGE_FILE_LARGE_ADDRESS_AWARE).to_ne_bytes();
    }
    file.seek_write(&bits, characteristics_offset).unwrap();
    file.flush().unwrap();
    // //PDFium 128.0.6569.0
    // let pdfium = Pdfium::new(
    //     Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
    //         .or_else(|_| Pdfium::bind_to_system_library()).unwrap(),
    // );
    //
    // // Load the document from the given path...
    //
    // let document = pdfium.load_pdf_from_file(r"D:\Downloads\eStatement_Jul2024.pdf", Option::None)
    //     .unwrap()
    //     .pages()
    //     .iter()
    //     .enumerate()
    //     .for_each(|(index, page)| {
    //         // For each page in the document, output the text on the page to the console.
    //
    //         println!("=============== Page {} ===============", index);
    //
    //         println!("{}", page.text().unwrap().all());
    //         // PdfPageText::all() returns all text across all page objects of type
    //         // PdfPageObjectType::Text on the page - this is convenience function,
    //         // since it is often useful to extract all the page text in one operation.
    //         // We could achieve exactly the same result by iterating over all the page
    //         // text objects manually and concatenating the text strings extracted from
    //         // each object together, like so:
    //
    //         // println!(
    //         //     "{}",
    //         //     page.objects()
    //         //         .iter()
    //         //         .filter_map(|object| object
    //         //             .as_text_object()
    //         //             .map(|object| object.text()))
    //         //         .collect::<Vec<_>>()
    //         //         .join("")
    //         // );
    //     });
    // let mut publisher = Publisher::default();
    // let subscription = Arc::new(Subscription::new(Box::new(|version| {
    //     println!("{}", version);
    // })));
    //
    // publisher.subscribe(Event::LatestVersion, subscription.clone());
    // publisher.notify(Event::LatestVersion, BuildVersion::default());
    // publisher.unsubscribe(Event::LatestVersion, subscription.clone());
    // publisher.notify(Event::LatestVersion, BuildVersion::default());
}
