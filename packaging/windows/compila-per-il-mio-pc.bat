@echo off
rem Compila FilmCraft apposta per il processore di questo PC (usa le istruzioni
rem AVX2 e simili, che la compilazione standard non usa).
rem
rem IMPORTANTE: il programma che ne esce funziona SOLO su processori uguali o
rem piu' recenti di questo. Non copiarlo su un altro PC. Se cambi processore,
rem ricompilalo.
rem
rem Il tuo FilmCraft attuale non viene toccato: il nuovo programma esce in
rem target\release\filmcraft.exe

setlocal
cd /d "%~dp0\..\.."

where cargo >nul 2>nul
if errorlevel 1 (
    echo.
    echo Non trovo Rust ^(cargo^). Installalo da https://rustup.rs e riapri questo file.
    echo.
    pause
    exit /b 1
)

echo.
echo Compilo FilmCraft per questo PC. La prima volta ci vuole molto tempo
echo ^(anche piu' di mezz'ora^). Non chiudere la finestra.
echo.

set RUSTFLAGS=-C target-cpu=native
cargo build --release -p filmcraft
if errorlevel 1 (
    echo.
    echo La compilazione e' fallita. Copia gli ultimi messaggi qui sopra e mandameli.
    echo.
    pause
    exit /b 1
)

echo.
echo Fatto. Il programma e' in: %CD%\target\release\filmcraft.exe
echo.
pause
