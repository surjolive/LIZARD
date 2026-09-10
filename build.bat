@echo off
setlocal
set ROOT=%~dp0
set CARGO=%USERPROFILE%\.cargo\bin\cargo.exe
if not exist "%CARGO%" set CARGO=cargo
if exist "C:\mingw64\bin" set PATH=C:\mingw64\bin;%PATH%
if not exist "%ROOT%release\windows-x64" mkdir "%ROOT%release\windows-x64"
%CARGO% fmt -- --check || exit /b 1
%CARGO% check || exit /b 1
%CARGO% build --release --target x86_64-pc-windows-gnu || exit /b 1
copy /Y "%ROOT%target\x86_64-pc-windows-gnu\release\lizard.exe" "%ROOT%release\windows-x64\lizard.exe" >nul
copy /Y "%ROOT%target\x86_64-pc-windows-gnu\release\lz.exe" "%ROOT%release\windows-x64\lz.exe" >nul
where certutil >nul 2>nul && certutil -hashfile "%ROOT%release\windows-x64\lz.exe" SHA256 > "%ROOT%release\windows-x64\lz.exe.sha256"
where certutil >nul 2>nul && certutil -hashfile "%ROOT%release\windows-x64\lizard.exe" SHA256 > "%ROOT%release\windows-x64\lizard.exe.sha256"
echo BUILD SUCCESSFUL
echo LIZARD CLI: %ROOT%release\windows-x64\lz.exe
echo LIZARD: %ROOT%release\windows-x64\lizard.exe
