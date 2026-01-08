<!-- @format -->

# Standalone Desktop App Deployment Guide

## Overview

This application is designed as a **standalone desktop app** with an **embedded SQLite database**. No external database server is required - everything users need is bundled in the application.

## How It Works

### Database Location

The application automatically creates and manages the database in the user's **platform-specific application data directory**:

| Platform    | Database Location                                                      |
| ----------- | ---------------------------------------------------------------------- |
| **macOS**   | `~/Library/Application Support/Income Calculator/income_calculator.db` |
| **Windows** | `%APPDATA%\Income Calculator\income_calculator.db`                     |
| **Linux**   | `~/.local/share/income-calculator/income_calculator.db`                |

### First Launch Behavior

When a user first launches the application:

1. ✅ The app automatically creates the application data directory
2. ✅ Creates a new SQLite database file
3. ✅ Runs all migrations to set up the database schema
4. ✅ The app is ready to use - no configuration needed!

### Benefits for End Users

- **Zero configuration** - Just download and run
- **No admin rights required** - Database is in user's own directory
- **Persistent data** - Data survives app updates
- **Cross-platform** - Same experience on Mac, Windows, and Linux
- **Backup-friendly** - Users can easily find and backup their data file

## Development vs Production

### Development Mode

During development, you can override the database location using an environment variable:

```bash
export DATABASE_URL="sqlite://./income_calculator.db?mode=rwc"
```

This places the database in your current directory for easier testing.

### Production Mode

In production builds, the app automatically uses the platform-specific directory. No environment variables needed!

## Building for Distribution

### Build the Release Binary

```bash
# For your current platform
cd frontend
dx build --release --platform desktop

# The binary will be in: target/desktop-release/
```

### Platform-Specific Builds

#### macOS

```bash
dx build --release --platform desktop
# Creates a .app bundle that can be distributed
```

#### Windows (cross-compile from Mac)

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
dx build --release --platform desktop --target x86_64-pc-windows-gnu
```

#### Linux

```bash
# Install Linux target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux
dx build --release --platform desktop --target x86_64-unknown-linux-gnu
```

## Testing the Standalone Behavior

### Test in Development

1. Remove any existing `DATABASE_URL` environment variable:

   ```bash
   unset DATABASE_URL
   ```

2. Run the app:

   ```bash
   cd frontend
   dx serve --platform desktop
   ```

3. Check where the database was created:

   ```bash
   # macOS
   ls -la ~/Library/Application\ Support/Income\ Calculator/

   # Windows
   dir %APPDATA%\Income Calculator\

   # Linux
   ls -la ~/.local/share/income-calculator/
   ```

### Test the Release Build

```bash
cd frontend
dx build --release --platform desktop

# Run the release binary directly
./target/desktop-release/income_calculator  # Linux/Mac
# or
.\target\desktop-release\income_calculator.exe  # Windows
```

## Distributing to Users

### Simple Distribution

1. Build the release binary for the target platform
2. Package the binary (optionally with a README)
3. Users just download and run - that's it!

### Professional Distribution

For a more polished experience, consider:

- **macOS**: Create a `.dmg` installer with drag-to-Applications
- **Windows**: Use tools like `wix` or `inno-setup` to create an installer
- **Linux**: Create `.deb` or `.rpm` packages

## Data Backup & Migration

### Where Users Can Find Their Data

Instruct users to backup this file:

**macOS:**

```
~/Library/Application Support/Income Calculator/income_calculator.db
```

**Windows:**

```
%APPDATA%\Income Calculator\income_calculator.db
```

**Linux:**

```
~/.local/share/income-calculator/income_calculator.db
```

### Restoring Data

Users can restore their data by:

1. Close the application
2. Replace the database file with their backup
3. Restart the application

## Troubleshooting

### Database Won't Create

If users report the database won't create:

- Check directory permissions in the app data folder
- Ensure enough disk space is available
- Check antivirus isn't blocking file creation

### Can't Find Database

The app automatically detects the correct location. If issues occur:

1. Check the error message in the app (it shows the full path)
2. Verify the directory exists and is writable
3. Try running with elevated permissions (rarely needed)

### Migrating from Old Location

If you previously used a different database location, users can:

1. Find their old database file
2. Copy it to the new standard location
3. Restart the app

## Security Considerations

- **No network access required** - All data stays local
- **Standard file permissions** - Database uses OS file security
- **User-only access** - Database is in user's private directory
- **No sensitive credentials** - No database passwords needed

## Future Updates

When you release app updates:

- ✅ Database migrations run automatically on first launch
- ✅ User data is preserved
- ✅ No manual steps required from users
- ✅ Rollback capability via database migrations

## Summary

This setup provides the **easiest possible experience for end users**:

1. **Download** → **Run** → **Use**
2. No configuration, no setup, no database server
3. Data automatically stored in the right place
4. Works the same on all platforms
5. Simple backup and restore

Perfect for standalone desktop applications! 🚀
