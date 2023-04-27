#!/bin/sh
#
# COPY OF /Applications/Platypus.app/Contents/Resources
# with replaced variables
#
# InstallCommandLineTool.sh
# Platypus
#
# Created by Sveinbjorn Thordarson on 6/17/08.
# Variables defined in Common.h

REAL_USER_ID=`/usr/bin/id -r -u`

echo "Installing command line tool"

# Create directories if they don't exist
echo "Creating directory structures"
mkdir -p "/usr/local/bin"
mkdir -p "/usr/local/share/platypus"
mkdir -p "/usr/local/share/man/man1"

# Change to Resources directory of Platypus application, which is first argument
echo "Changing to directory '$1'"
cd "$1"

echo "Copying resources to share directory"
# ScriptExec binary
gunzip -c "ScriptExec.gz" > "/usr/local/share/platypus/ScriptExec"
# Nib
cp -r MainMenu.nib "/usr/local/share/platypus"
# Set permissions
chown -R ${REAL_USER_ID} "/usr/local/share/platypus"
chmod -R 755 "/usr/local/share/platypus"

# Command line tool binary
echo "Installing command line tool"
gunzip -c "platypus_clt.gz" > "/usr/local/bin/platypus"
chown ${REAL_USER_ID} "/usr/local/bin/platypus"
chmod +x "/usr/local/bin/platypus"

# Man page
echo "Installing man page"
rm "/usr/local/share/man/man1/platypus.1.gz" &> /dev/null
rm "/usr/local/share/man/man1/platypus.1.gz.gz" &> /dev/null
cp "platypus.1.gz" "/usr/local/share/man/man1/platypus.1.gz"
chmod 644 "/usr/local/share/man/man1/platypus.1.gz"
chown ${REAL_USER_ID} "/usr/local/share/man/man1/platypus.1.gz"

exit 0
