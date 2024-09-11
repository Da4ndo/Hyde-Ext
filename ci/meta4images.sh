#!/bin/bash

# Function to generate source_urls from a given folder
generate_source_urls() {
  local folder_path=$1
  local github_prefix=$2
  local source_urls=""

  if [ ! -d "$folder_path" ]; then
    echo -e "\e[31mError:\e[0m The provided path is not a directory."
    exit 1
  fi

  for file in "$folder_path"/*; do
    if [ -f "$file" ]; then
      local file_name=$(basename "$file")
      source_urls+="${github_prefix}${file_name}|"
    fi
  done

  # Remove the trailing '|'
  source_urls=${source_urls%|}

  echo "$source_urls"
}

# Main script execution
if [ $# -ne 2 ]; then
  echo -e "\e[31mError:\e[0m Please provide exactly one folder path and one GitHub link prefix as arguments."
  exit 1
fi

folder_path=$1
github_prefix=$2
source_urls=$(generate_source_urls "$folder_path" "$github_prefix")

if [ -z "$source_urls" ]; then
  echo -e "\e[31mError:\e[0m No files found in the provided directory."
  exit 1
fi

echo "Generated source_urls: $source_urls"
