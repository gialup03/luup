#!/bin/bash
echo "Testing Rust compilation..."
cd src-tauri
cargo check 2>&1 | tee ../compile-output.txt
exit_code=${PIPESTATUS[0]}

if [ $exit_code -eq 0 ]; then
    echo ""
    echo "✅ Compilation successful!"
    echo "Run: npm run tauri:dev"
else
    echo ""
    echo "❌ Compilation failed. See compile-output.txt for details"
    tail -50 ../compile-output.txt
fi

exit $exit_code

