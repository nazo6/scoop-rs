# [wip] scoop-rs

Rust rewrite of the Scoop package manager

## Goals

- Rewrite Scoop in Rust with async for better performance
- Full compatibility with existing Scoop manifests, buckets and directory
  structure
- Develop TUI for managing packages
- No database. Filesystem is the only source of truth

## Non-goals

- CLI compatibility with Scoop
