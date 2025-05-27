use csv::{ReaderBuilder, WriterBuilder};
use rfd::FileDialog;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn convert_csv(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let input = File::open(input_file)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(input);

    let output = File::create(output_file)?;
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(output);

    if let Some(headers) = reader
        .headers()?
        .iter()
        .map(|h| h.replace('.', ","))
        .collect::<Vec<String>>()
        .into()
    {
        writer.write_record(&headers)?;
    }

    for result in reader.records() {
        let record = result?;
        let converted_record: Vec<String> =
            record.iter().map(|cell| cell.replace('.', ",")).collect();
        writer.write_record(&converted_record)?;
    }

    Ok(())
}

fn main() {
    let input_file = FileDialog::new()
        .add_filter("CSV files", &["csv"])
        .set_title("Seleccione el archivo CSV de entrada")
        .pick_file()
        .expect("No se seleccionó ningún archivo")
        .to_str()
        .expect("Error al convertir la ruta del archivo")
        .to_string();

    let output_folder = FileDialog::new()
        .set_title("Seleccione la carpeta para guardar el archivo CSV convertido")
        .pick_folder()
        .expect("No se seleccionó ninguna carpeta")
        .to_str()
        .expect("Error al convertir la ruta de la carpeta")
        .to_string();

    let input_file_name = Path::new(&input_file)
        .file_name()
        .expect("Error al obtener el nombre del archivo de entrada")
        .to_str()
        .expect("Error al convertir el nombre del archivo de entrada");

    let output_file = Path::new(&output_folder).join(input_file_name);

    match convert_csv(&input_file, output_file.to_str().unwrap()) {
        Ok(_) => println!(
            "Archivo convertido guardado en: {}",
            output_file.to_str().unwrap()
        ),
        Err(e) => eprintln!("Error al convertir el archivo: {}", e),
    }
}
