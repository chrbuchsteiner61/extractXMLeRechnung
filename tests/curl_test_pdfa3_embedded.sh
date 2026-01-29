#!/bin/bash

# Test curl file for aTextFile_pdfa3_embedded.pdf
# This PDF/A-3 file contains embedded XML content

API_BASE="${API_BASE:-http://127.0.0.1:8080}"
PDF_FILE="examples/aTextFile_pdfa3_embedded.pdf"

echo "🔍 Testing extract_xml endpoint with PDF/A-3 embedded XML"
echo "=========================================================="
echo "File: $PDF_FILE"
echo "Expected: Success response with extracted XML content"
echo ""

# Check if PDF file exists
if [ ! -f "$PDF_FILE" ]; then
    echo "❌ Error: PDF file '$PDF_FILE' does not exist"
    echo "Please run this script from the project root directory"
    exit 1
fi

echo "📄 Sending request..."
curl -s -X POST \
     -F "file=@$PDF_FILE" \
     -H "Accept: application/json" \
     "$API_BASE/extract_xml" | jq .

echo ""
echo "✅ Test completed"