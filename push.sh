#!/bin/bash

# Add all changes to the staging area
git add .

git status

echo -n "Enter your commit message: "
read commit_message
git commit -m "$commit_message"

git push

