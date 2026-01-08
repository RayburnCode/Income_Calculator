<!-- @format -->

# Income Calculator - Packaging and Distribution Guide

This guide explains how to package the Income Calculator application with Dioxus, set up database connectivity for distribution, and create distributable binaries for Mac and Windows.

## Overview

The Income Calculator is a Dioxus-based desktop application that uses SQLite for data persistence. This guide covers:

1. Database setup for distributable applications
2. Dioxus desktop packaging
3. Cross-platform builds (Mac & Windows)
4. Database connectivity in the binary

## Project Structure

```
Income_Calculator/
├── database/           # Database layer (SeaORM + SQLite)
├── client/            # Business logic
├── shared/            # Shared models and types
├── frontend/          # Dioxus desktop application
└── target/            # Build artifacts
```

## Database Setup for Distribution

### Using SQLite for Distribution

For desktop applications, we use SQLite as the database engine because:

- **No external dependencies**: SQLite is embedded in the binary
- **Cross-platform**: Works identically on Mac, Windows, and Linux
- **File-based**: Easy to backup and distribute
- **ACID compliant**: Reliable data storage

### Database Configuration

The database connection is configured in `database/.env`:

```env
# Database configuration for production/distribution
DATABASE_URL=sqlite://./income_calculator.db?mode=rwc
```

**Important**: The `?mode=rwc` parameter ensures:

- `r`: Read access
- `w`: Write access
- `c`: Create database if it doesn't exist

### Database Initialization

The application automatically runs migrations on startup. The migration system creates all necessary tables:

1. **Client Info**: `borrowers` table
2. **Income Worksheets**: `income_information`, `consumer_debts`
3. **Options Templates**: All mortgage refinance tables

## Dioxus Desktop Packaging

### Dioxus Configuration

The Dioxus desktop configuration is in `frontend/Dioxus.toml`:

```toml
[application]

# App name
name = "Income Calculator"

# App identifier (reverse domain notation)
identifier = "com.incomecalculator.app"

# App version
version = "0.1.0"

# Asset directory
assets = "assets"

[web.app]

# Title of the application
title = "Income Calculator"

[web.watcher]

# Where to watch for changes
watch_path = "src"

# Files to reload when changed
reload_html = true

[web.resource]

# Development server port
port = 8080

[web.resource.dev]

# CORS policy for development
cors = true

[desktop]

# Window title
title = "Income Calculator"

# Window size
width = 1200
height = 800

# Minimum window size
min_width = 800
min_height = 600

# Enable window decorations
decorations = true

# Enable transparency
transparent = false

# Window icon (PNG)
icon = "assets/icon.png"
```

### Building for Distribution

#### Mac Build

```bash
# Build for Mac (Intel)
cd frontend
dx build --release --platform macos

# Build for Mac (Apple Silicon)
dx build --release --platform macos --arch aarch64
```

#### Windows Build

```bash
# Build for Windows (64-bit)
cd frontend
dx build --release --platform windows --arch x86_64
```

#### Linux Build

```bash
# Build for Linux
cd frontend
dx build --release --platform linux --arch x86_64
```

### Output Locations

Built applications are placed in:

- **Mac**: `frontend/target/dx/frontend/release/bundle/macos/`
- **Windows**: `frontend/target/dx/frontend/release/bundle/windows/`
- **Linux**: `frontend/target/dx/frontend/release/bundle/linux/`

## Database Connection in Binary

### Database Service Architecture

The application uses a layered architecture for database access:

```
Frontend (Dioxus) → Client Service → Database Layer → SQLite
```

### Database Service Implementation

The database connection is managed in `database/src/lib.rs`:

```rust
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./income_calculator.db?mode=rwc".to_string());

    Database::connect(&database_url).await
}
```

### Migration on Startup

The application automatically runs migrations when it starts. This is handled in the main application entry point:

```rust
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Establish database connection
    let db = database::establish_connection().await?;

    // Run migrations
    Migrator::up(&db, None).await?;

    // Start the application
    dioxus_desktop::launch(App);

    Ok(())
}
```

## Cross-Platform Distribution

### Mac Distribution

#### Creating a .dmg file

```bash
# Install create-dmg (if not already installed)
brew install create-dmg

# Create DMG from the app bundle
create-dmg \
  --volname "Income Calculator" \
  --volicon "assets/icon.icns" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Income Calculator.app" 200 190 \
  --hide-extension "Income Calculator.app" \
  --app-drop-link 600 185 \
  "Income-Calculator.dmg" \
  "target/dx/frontend/release/bundle/macos/Income Calculator.app"
```

#### Notarization (for App Store distribution)

```bash
# Create keychain profile
xcrun notarytool store-credentials "notary-profile" \
  --apple-id "your-apple-id@example.com" \
  --team-id "YOUR_TEAM_ID" \
  --password "app-specific-password"

# Submit for notarization
xcrun notarytool submit "Income-Calculator.dmg" \
  --keychain-profile "notary-profile" \
  --wait

# Staple the notarization ticket
xcrun stapler staple "Income-Calculator.dmg"
```

### Windows Distribution

#### Creating an installer with NSIS

1. Install NSIS:

```bash
# Using Chocolatey
choco install nsis
```

2. Create installer script (`installer.nsi`):

```nsis
!include "MUI2.nsh"

Name "Income Calculator"
OutFile "Income-Calculator-Setup.exe"
Unicode True

InstallDir "$PROGRAMFILES\Income Calculator"
InstallDirRegKey HKCU "Software\IncomeCalculator" ""

!define MUI_ABORTWARNING

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "MainSection" SEC01
    SetOutPath "$INSTDIR"
    File /r "target\dx\frontend\release\bundle\windows\*.*"

    # Create desktop shortcut
    CreateShortCut "$DESKTOP\Income Calculator.lnk" "$INSTDIR\income_calculator.exe"

    # Registry information for add/remove programs
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\IncomeCalculator" "DisplayName" "Income Calculator"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\IncomeCalculator" "UninstallString" "$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\IncomeCalculator" "DisplayVersion" "1.0.0"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\IncomeCalculator" "Publisher" "Your Company"

    WriteUninstaller "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\uninstall.exe"
    Delete "$DESKTOP\Income Calculator.lnk"
    RMDir /r "$INSTDIR"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\IncomeCalculator"
SectionEnd
```

3. Compile the installer:

```bash
makensis installer.nsi
```

#### Creating a .zip archive

```bash
# Create zip archive
cd target/dx/frontend/release/bundle/windows
zip -r Income-Calculator-Windows.zip .
```

### Linux Distribution

#### Creating an AppImage

```bash
# Install appimagetool
wget "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
chmod +x appimagetool-x86_64.AppImage

# Create AppDir structure
mkdir -p Income-Calculator.AppDir
cp -r target/dx/frontend/release/bundle/linux/* Income-Calculator.AppDir/

# Create AppRun script
cat > Income-Calculator.AppDir/AppRun << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
exec ./income_calculator "$@"
EOF
chmod +x Income-Calculator.AppDir/AppRun

# Create desktop file
cat > Income-Calculator.AppDir/income_calculator.desktop << EOF
[Desktop Entry]
Name=Income Calculator
Exec=income_calculator
Icon=income_calculator
Type=Application
Categories=Office;Finance;
EOF

# Build AppImage
./appimagetool-x86_64.AppImage Income-Calculator.AppDir
```

## Build Scripts

### Automated Build Script

Create a build script (`build.sh` or `build.ps1`) for automated cross-platform builds:

```bash
#!/bin/bash
# build.sh

echo "Building Income Calculator for multiple platforms..."

# Clean previous builds
rm -rf target/dx

# Build for current platform
echo "Building for native platform..."
cd frontend
dx build --release

# Build for other platforms (requires cross-compilation setup)
echo "Building for Windows..."
dx build --release --platform windows --arch x86_64

echo "Building for Linux..."
dx build --release --platform linux --arch x86_64

echo "Build complete!"
```

### CI/CD with GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: macos-latest
            target: macos
            arch: x86_64
          - os: macos-latest
            target: macos
            arch: aarch64
          - os: windows-latest
            target: windows
            arch: x86_64
          - os: ubuntu-latest
            target: linux
            arch: x86_64

    steps:
      - uses: actions/checkout@v3

      - name: Setup Dioxus
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.arch != 'x86_64' && format('{0}-apple-darwin', matrix.arch) || '' }}

      - name: Build
        run: |
          cd frontend
          dx build --release --platform ${{ matrix.target }} --arch ${{ matrix.arch }}

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: income-calculator-${{ matrix.target }}-${{ matrix.arch }}
          path: frontend/target/dx/frontend/release/bundle/${{ matrix.target }}/
```

## Database Backup and Data Management

### Automatic Backup

Implement automatic backup functionality:

```rust
use std::fs;
use std::path::Path;

pub fn backup_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "income_calculator.db";
    let backup_path = format!("backup_{}.db", chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    fs::copy(db_path, backup_path)?;
    println!("Database backed up to: {}", backup_path);
    Ok(())
}
```

### Data Export/Import

Add data export functionality for user data portability:

```rust
use serde_json;
use std::fs::File;
use std::io::Write;

pub async fn export_data(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Export all borrowers with related data
    let borrowers = Borrower::find()
        .find_with_related(IncomeInformation)
        .find_with_related(ConsumerDebt)
        .all(db)
        .await?;

    let json = serde_json::to_string_pretty(&borrowers)?;
    let mut file = File::create("export.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Database file not found**:

   - Ensure the application has write permissions in the installation directory
   - Check the `DATABASE_URL` environment variable

2. **Migration failures**:

   - Delete the database file and restart the application
   - Check migration logs for specific errors

3. **Cross-platform compatibility**:
   - Use relative paths for database files
   - Avoid platform-specific file paths
   - Test on target platforms during development

### Performance Optimization

1. **Database connection pooling**:

   - Use connection pooling for better performance
   - Configure appropriate pool sizes

2. **Query optimization**:

   - Use database indexes on frequently queried columns
   - Implement pagination for large datasets

3. **Memory management**:
   - Stream large result sets
   - Use appropriate data types

## Security Considerations

1. **Database encryption**: Consider SQLCipher for sensitive data
2. **Input validation**: Validate all user inputs
3. **SQL injection**: Use parameterized queries (SeaORM handles this)
4. **File permissions**: Restrict database file permissions
5. **Backup security**: Encrypt backup files

## Next Steps

1. Set up automated testing for database operations
2. Implement user authentication and authorization
3. Add data synchronization capabilities
4. Create update mechanism for distributed applications
5. Implement telemetry and error reporting

---

This guide provides a comprehensive overview of packaging and distributing the Income Calculator application. For specific implementation details, refer to the source code and Dioxus documentation.
