@echo off

:: Examples: empty, exitwithcode, functions
set example_name=functions

echo Building compiler project
cargo build
if %errorlevel% neq 0 exit /b %errorlevel%

echo Compiling YYN to LLVM IR
cargo run -- programs\%example_name%.yyn
if %errorlevel% neq 0 exit /b %errorlevel%

echo Compiling LLVM IR to object code
llc programs\%example_name%.ll
if %errorlevel% neq 0 exit /b %errorlevel%

echo Linking object code
gcc programs\%example_name%.s -mwindows -o programs\%example_name%.exe
if %errorlevel% neq 0 exit /b %errorlevel%

echo Executing compiled YYN program
".\programs\%example_name%.exe"
echo Exited with code %errorlevel%
