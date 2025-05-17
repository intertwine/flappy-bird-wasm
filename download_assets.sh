#!/bin/bash

# Create assets directory if it doesn't exist
mkdir -p assets

# Download bird sprite
curl -o assets/bird.png "https://raw.githubusercontent.com/sourabhv/FlapPyBird/master/assets/sprites/yellowbird-midflap.png"

# Download pipe sprite
curl -o assets/pipe.png "https://raw.githubusercontent.com/sourabhv/FlapPyBird/master/assets/sprites/pipe-green.png"

# Download background
curl -o assets/background.png "https://raw.githubusercontent.com/sourabhv/FlapPyBird/master/assets/sprites/background-day.png"

echo "Assets downloaded successfully!"
