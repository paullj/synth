#!/bin/bash

set -eou pipefail

DEVICE_USER="$1"
DEVICE_HOST="$2"
SERVICE_NAME="$3"
SERVICE_FILE="$4"
TARGET_DIR="$5"

# Copy the service file to the Raspberry Pi
scp "$SERVICE_FILE" "$DEVICE_USER@$DEVICE_HOST:$TARGET_DIR/$SERVICE_NAME.service"

# SSH into the Raspberry Pi and enable the service
# Set permissions and enable the service
# Check if the service is already enabled and if not enable it
# Reload the systemd daemon and start the service
ssh "$DEVICE_USER@$DEVICE_HOST" << EOF
  chmod 644 $TARGET_DIR/*.service
  if [ ! -L /etc/systemd/system/$SERVICE_NAME.service ]; then
    sudo ln -s $TARGET_DIR/$SERVICE_NAME.service /etc/systemd/system/
  fi
  sudo systemctl daemon-reload
  sudo systemctl enable $SERVICE_NAME.service
  sudo systemctl start $SERVICE_NAME.service
EOF

echo "Service $SERVICE_NAME has been added and started on $DEVICE_HOST"
