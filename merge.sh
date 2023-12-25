#!/bin/bash

# Directory of the new repository
NEW_REPO_DIR="./"

# Array of paths to the local repositories to merge
REPOS_DIR='./days'
REPO_PATHS=($(ls -d "$REPOS_DIR"/*))

# Move to the new repository directory
cd "$NEW_REPO_DIR"

# Loop through each repository
for REPO in "${REPO_PATHS[@]}"; do
    # Extract the name of the repository from the path
    REPO_NAME=$(basename "$REPO")

    # Add as a subdirectory
    cd "$REPO"
    git filter-repo --to-subdirectory-filter "$REPO_NAME" --force

    # Go back
    cd ../..

    # Add the local repository as a remote
    git remote add "$REPO_NAME" "$REPO"

    # Fetch the data from the repository
    git fetch "$REPO_NAME"

    # Merge the branch into the master branch
    git merge --allow-unrelated-histories "$REPO_NAME"/master -m "Added $REPO_NAME"

    # Remove the remote to clean up
    git remote remove "$REPO_NAME"
done
