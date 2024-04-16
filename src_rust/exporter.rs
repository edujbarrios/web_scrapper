use calamine::{Writer, Xlsx};
use docx_rs::{Docx, Paragraph, Run, RunFonts, Size};
use printpdf::{PdfDocument, PdfLayerReference, Mm};
use serde_json::to_string_pretty;
use std::{fs::File, io::BufWriter};

pub fn export_to_excel(data: &[String], filename: &str) {
    let mut workbook = Xlsx::create(filename).unwrap();
    let mut sheet = workbook.create_sheet("Sheet1");

    for (index, value) in data.iter().enumerate() {
        sheet.write_string((index as u32, 0), value, None).unwrap();
    }

    workbook.close().unwrap();
}

pub fn export_to_word(data: &[String], filename: &str) {
    let mut doc = Docx::new();

    for line in data {
        let run = Run::new().add_text(line);
        let para = Paragraph::new().add_run(run);
        doc = doc.add_paragraph(para);
    }

    doc.build().pack(filename).unwrap();
}

pub fn export_to_pdf(data: &[String], filename: &str) {
    let (doc, page1, layer1) = PdfDocument::new("PDF Document", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer: PdfLayerReference = doc.get_page(page1).get_layer(layer1);

    for (index, text) in data.iter().enumerate() {
        current_layer.use_text(text, 14, Mm(10.0), Mm(297.0 - 10.0 * (index as f64 + 1.0)), &printpdf::built_in_fonts::HELVETICA);
    }

    doc.save(&mut BufWriter::new(File::create(filename).unwrap())).unwrap();
}

pub fn export_to_json(data: &[String], filename: &str) {
    let json = to_string_pretty(data).unwrap();
    std::fs::write(filename, json).expect("Failed to write to file");
}
