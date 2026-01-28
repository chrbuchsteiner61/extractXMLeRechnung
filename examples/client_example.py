#!/usr/bin/env python3
"""
Example client for the eRechnung PDF/A-3 XML Extractor API

This script demonstrates how to use the API to extract XML content from PDF files.
"""

import json
import sys
from pathlib import Path

import requests


def extract_xml_from_pdf(api_url: str, pdf_file_path: str) -> dict:
    """
    Extract XML from a PDF file using the API
    
    Args:
        api_url: Base URL of the API (e.g., http://localhost:8080)
        pdf_file_path: Path to the PDF file to process
        
    Returns:
        Dictionary containing the API response
    """
    url = f"{api_url}/extract_xml"
    
    with open(pdf_file_path, 'rb') as f:
        files = {'file': (Path(pdf_file_path).name, f, 'application/pdf')}
        response = requests.post(url, files=files)
    
    return {
        'status_code': response.status_code,
        'response': response.json() if response.headers.get('content-type', '').startswith('application/json') else response.text
    }


def check_health(api_url: str) -> dict:
    """
    Check API health status
    
    Args:
        api_url: Base URL of the API
        
    Returns:
        Dictionary containing the health check response
    """
    url = f"{api_url}/health"
    response = requests.get(url)
    
    return {
        'status_code': response.status_code,
        'response': response.json() if response.headers.get('content-type', '').startswith('application/json') else response.text
    }


def main():
    api_url = "http://127.0.0.1:8080"
    
    # Check health first
    print("Checking API health...")
    health = check_health(api_url)
    print(f"Health check: {json.dumps(health, indent=2)}")
    
    if health['status_code'] != 200:
        print("API is not healthy. Please start the server first.")
        sys.exit(1)
    
    # Extract XML from PDF if file path provided
    if len(sys.argv) > 1:
        pdf_path = sys.argv[1]
        
        if not Path(pdf_path).exists():
            print(f"Error: File {pdf_path} does not exist")
            sys.exit(1)
            
        print(f"\nExtracting XML from {pdf_path}...")
        result = extract_xml_from_pdf(api_url, pdf_path)
        print(f"Result: {json.dumps(result, indent=2)}")
        
        # Save XML content if extraction was successful
        if (result['status_code'] == 200 and 
            isinstance(result['response'], dict) and 
            'xml_content' in result['response']):
            
            xml_filename = result['response'].get('xml_filename', 'extracted.xml')
            with open(xml_filename, 'w', encoding='utf-8') as f:
                f.write(result['response']['xml_content'])
            print(f"\nXML content saved to: {xml_filename}")
    else:
        print("\nUsage: python3 client_example.py <path_to_pdf_file>")
        print("Example: python3 client_example.py invoice.pdf")


if __name__ == "__main__":
    main()