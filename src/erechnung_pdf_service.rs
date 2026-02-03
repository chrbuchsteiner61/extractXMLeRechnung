use crate::errors::PDFError;
use crate::models::{ErrorResponse, SuccessResponse};
use crate::pdf_worker::{carveout_xml_from_pdf, EmbeddedFilesExtractor, PDFA3Validator};

/// Main business logic for eRechnung processing
pub struct ERechnungService;

impl ERechnungService {
    /// Process a PDF file and extract XML content
    pub fn process_pdf(pdf_bytes: Vec<u8>) -> Result<SuccessResponse, ErrorResponse> {
        Self::validate_pdf_format(&pdf_bytes)?;
        Self::validate_pdfa3_format(&pdf_bytes)?;
        
        let embedded_files = Self::find_embedded_files(&pdf_bytes)?;
        let xml_file = Self::find_xml_file(&embedded_files)?;
        let xml_contents = Self::extract_xml_contents(&pdf_bytes, &embedded_files)?;
        let xml_content = Self::select_best_xml_content(&xml_contents, &embedded_files)?;
        let status = Self::determine_status(&xml_file);

        Ok(SuccessResponse {
            file_status: status,
            embedded_files: embedded_files.join(", "),
            xml_content: xml_content.clone(),
            xml_filename: xml_file.clone(),
        })
    }

    /// Validate that the input bytes represent a valid PDF file
    fn validate_pdf_format(pdf_bytes: &[u8]) -> Result<(), ErrorResponse> {
        if pdf_bytes.len() < 5 || &pdf_bytes[0..5] != b"%PDF-" {
            return Err(ErrorResponse {
                file_status: PDFError::InvalidPDF.to_string(),
                embedded_files: None,
            });
        }
        Ok(())
    }

    /// Validate that the PDF is in PDF/A-3 format
    fn validate_pdfa3_format(pdf_bytes: &[u8]) -> Result<(), ErrorResponse> {
        if let Err(PDFError::NotPDFA3) = PDFA3Validator::validate(pdf_bytes) {
            return Err(ErrorResponse {
                file_status: PDFError::NotPDFA3.to_string(),
                embedded_files: None,
            });
        }
        Ok(())
    }

    /// Find embedded files in the PDF
    fn find_embedded_files(pdf_bytes: &[u8]) -> Result<Vec<String>, ErrorResponse> {
        let embedded_files = EmbeddedFilesExtractor::find_embedded_files(pdf_bytes);
        
        if embedded_files.is_empty() {
            return Err(ErrorResponse {
                file_status: PDFError::NoXMLFile.to_string(),
                embedded_files: None,
            });
        }
        
        Ok(embedded_files)
    }

    /// Find an XML file among the embedded files
    fn find_xml_file(embedded_files: &[String]) -> Result<String, ErrorResponse> {
        embedded_files
            .iter()
            .find(|name| name.to_lowercase().ends_with(".xml"))
            .cloned()
            .ok_or_else(|| ErrorResponse {
                file_status: PDFError::NoXMLFile.to_string(),
                embedded_files: Some(embedded_files.join(", ")),
            })
    }

    /// Extract XML content from the PDF
    fn extract_xml_contents(pdf_bytes: &[u8], embedded_files: &[String]) -> Result<Vec<String>, ErrorResponse> {
        carveout_xml_from_pdf(pdf_bytes).map_err(|_| ErrorResponse {
            file_status: PDFError::ExtractionFailed.to_string(),
            embedded_files: Some(embedded_files.join(", ")),
        })
    }

    /// Select the best XML content from multiple extracted contents
    fn select_best_xml_content(xml_contents: &[String], embedded_files: &[String]) -> Result<String, ErrorResponse> {
        xml_contents
            .iter()
            .find(|content| content.contains("<rsm:"))
            .or_else(|| xml_contents.first())
            .cloned()
            .ok_or_else(|| ErrorResponse {
                file_status: PDFError::ExtractionFailed.to_string(),
                embedded_files: Some(embedded_files.join(", ")),
            })
    }

    /// Determine the processing status based on XML filename
    fn determine_status(xml_file: &str) -> String {
        let is_facturx = xml_file.to_lowercase() == "factur-x.xml";
        if is_facturx {
            "Success".to_string()
        } else {
            "XML is not Factur-x.xml".to_string()
        }
    }
}
