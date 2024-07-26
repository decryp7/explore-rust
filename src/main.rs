use std::ops::Deref;
use std::sync::Arc;
use crate::build_version::BuildVersion;
use crate::publisher::{Event, Publisher, Subscription};
use pdfium_render::prelude::*;

mod publisher;
mod build_version;

fn main() {

    //PDFium 128.0.6569.0
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library()).unwrap(),
    );

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file("/Volumes/Data/Test/eStatement_Jul2024.pdf", Option::None)
        .unwrap()
        .pages()
        .iter()
        .enumerate()
        .for_each(|(index, page)| {
            // For each page in the document, output the text on the page to the console.

            println!("=============== Page {} ===============", index);

            println!("{}", page.text().unwrap().all());

            // PdfPageText::all() returns all text across all page objects of type
            // PdfPageObjectType::Text on the page - this is convenience function,
            // since it is often useful to extract all the page text in one operation.
            // We could achieve exactly the same result by iterating over all the page
            // text objects manually and concatenating the text strings extracted from
            // each object together, like so:

            // println!(
            //     "{}",
            //     page.objects()
            //         .iter()
            //         .filter_map(|object| object
            //             .as_text_object()
            //             .map(|object| object.text()))
            //         .collect::<Vec<_>>()
            //         .join("")
            // );
        });
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
