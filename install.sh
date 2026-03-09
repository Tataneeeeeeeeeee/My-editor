#!/bin/bash

# Create the directory for the editor if it doesn't exist
sudo mkdir -p /usr/local/share/my-editor

# Copy the editor binary to the directory and set permissions
sudo cp my-editor /usr/local/share/my-editor/
sudo chmod +x /usr/local/share/my-editor/my-editor
sudo ln -sf /usr/local/share/my-editor/my-editor /usr/local/bin/my-editor

# Copy the assets to the directory
sudo cp -r assets /usr/local/share/my-editor/.

# Copy the settings to the user's home directory
mkdir -p ~/.my-editor
cp -r .my-editor/* ~/.my-editor/.
