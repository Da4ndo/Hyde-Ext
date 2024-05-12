#!/bin/bash

device_name="Keychron K2"
project_folder="$HOME/scripts/"

config_file="${project_folder}.current_config"
lang_config="${project_folder}lang.conf"
log_file="${project_folder}config_change.log"

# Ensure files exist
touch "$config_file"
touch "$lang_config"
touch "$log_file"

pc_config_content="input {
    kb_layout = us
    follow_mouse = 1

    touchpad {
        natural_scroll = no
    }

    sensitivity = 1 # -1.0 - 1.0, 0 means no modification.
    force_no_accel = 1
}"

laptop_config_content="input {
    kb_layout = hu
    follow_mouse = 1

    touchpad {
        natural_scroll = no
    }

    sensitivity = 1 # -1.0 - 1.0, 0 means no modification.
    force_no_accel = 1
}"

cleanup_logs() {
  local log_path=$1
  local max_lines=1000
  local cutoff_date=$(date -d '2 months ago' +%s)

  # Logging initiation message to both console and log file
  echo "$(date '+%Y-%m-%d %H:%M:%S') :: Initiating log cleanup process." >> "$log_path"

  # Create a temporary file to store the filtered logs
  local temp_log=$(mktemp)

  # Read each line in the log file
  while IFS= read -r line; do
    # Extract the date from the log line
    local log_date=$(echo "$line" | awk '{print $1}')
    local log_date_seconds=$(date -d "$log_date" +%s)

    # Check if the log date is within the last two months
    if [ "$log_date_seconds" -ge "$cutoff_date" ]; then
      echo "$line" >> "$temp_log"
    fi
  done < "$log_path"

  # Check if the temporary log exceeds the maximum allowed lines
  local line_count=$(wc -l < "$temp_log")
  if [ "$line_count" -gt "$max_lines" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S')   => Log file exceeds $max_lines lines. Trimming older entries." >> "$log_path"
    tail -n "$max_lines" "$temp_log" > "$log_path"
  else
    cat "$temp_log" > "$log_path"
  fi

  # Append completion message to the temporary log before moving it to the actual log file
  echo "$(date '+%Y-%m-%d %H:%M:%S') :: Log cleanup process completed." >> "$log_path"

  # Clean up
  rm "$temp_log"
}

# Schedule the cleanup_logs function to run at the start of the script
cleanup_logs "$log_file"


change_config() {
  local new_config_content=$1
  local config_type=$2
  echo "$new_config_content" > $lang_config
  echo "$(date '+%Y-%m-%d %H:%M:%S') :: Configuration changed to $config_type successfully." >> $log_file
  echo ":: Configuration changed successfully."
}

echo "$(date '+%Y-%m-%d %H:%M:%S') :: Script started." >> $log_file
echo ":: Script started."

# Ensure config file exists and has default content if not
if [ ! -f "$config_file" ]; then
  echo "$pc_config_content" > "$config_file"
  echo "$(date '+%Y-%m-%d %H:%M:%S') :: Initial configuration set to PC." >> "$log_file"
fi

while true; do
  devices=$(bluetoothctl devices Connected)
  current_config=$(cat $config_file)

  if [[ $devices =~ $device_name ]]; then
    if [[ $current_config != "pc" ]]; then
      echo "  => Changing to PC configuration."
      change_config "$pc_config_content" "pc"
      echo "pc" > $config_file
    fi
  else
    if [[ $current_config != "laptop" ]]; then
      echo "  => Changing to Laptop configuration."
      change_config "$laptop_config_content" "laptop"
      echo "laptop" > $config_file
    fi
  fi

  sleep 10
done