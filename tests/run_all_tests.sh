#!/bin/bash

# Master test script for extract_xml endpoint
# Runs all PDF test cases sequentially

set -e

API_BASE="${API_BASE:-http://127.0.0.1:8080}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 eRechnung PDF XML Extractor - API Test Suite"
echo "================================================="
echo "API Base URL: $API_BASE"
echo "Project Root: $PROJECT_ROOT"
echo ""

# Change to project root for relative paths to work
cd "$PROJECT_ROOT"

# Function to run test and capture results
run_test() {
    local test_script="$1"
    local test_name="$2"
    
    echo "🔄 Running: $test_name"
    echo "----------------------------------------"
    
    if [ -f "$test_script" ]; then
        bash "$test_script"
    else
        echo "❌ Test script not found: $test_script"
        return 1
    fi
    
    echo ""
    echo "----------------------------------------"
    echo ""
}

# Health check first
echo "🏥 Health Check"
echo "==============="
curl -s "$API_BASE/health" | jq .
echo ""
echo ""

# Run all tests
run_test "tests/curl_test_ohne_embedded.sh" "PDF without embedded XML"
run_test "tests/curl_test_pdfa3_embedded.sh" "PDF/A-3 with embedded XML"
run_test "tests/curl_test_pdfa3_xml_embedded.sh" "PDF-3 with XML embedded"

echo "🏁 All tests completed!"
echo "======================="