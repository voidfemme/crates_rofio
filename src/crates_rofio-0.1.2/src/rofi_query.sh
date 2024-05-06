#!/bin/bash

QUERY=""
PREV_QUERY=""
RESULTS=""

while true ; do
  QUERY=$(echo "$RESULTS" | rofi -dmenu -p "Search Crates:" -matchin fuzzy -filter "$QUERY")

  # Check if the query has changed
  if [ "$QUERY" != "$PREV_QUERY" ]; then
    PREV_QUERY=$QUERY
    # Call Rust app to get new results
    RESULTS=$(/home/rsp/scripts/rust/rust_man/target/debug/rust_man "$QUERY")
  else
    # exit if the user selects an option or cancels
    break
  fi
done

echo "$RESULTS"
