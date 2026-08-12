#!/usr/bin/env bash

# ------------------------------------------------------------
# Build script
#
# Steps: backend, frontend, docker (docker always runs).
#
# Usage:
#   ./build.sh                          # run all steps
#   ./build.sh --skip-backend           # skip backend
#   ./build.sh --skip-frontend          # skip frontend
#   ./build.sh --skip-backend --skip-frontend   # only docker
#   ./build.sh --only backend           # run only backend + docker
#   ./build.sh --only frontend          # run only frontend + docker
#   ./build.sh -h | --help              # show help
# ------------------------------------------------------------

set -euo pipefail

PROJECT_PATH="$(pwd)"

RUN_BACKEND=true
RUN_FRONTEND=true

show_help() {
    echo "Usage: build.sh [options]"
    echo ""
    echo "Options:"
    echo "  --skip-backend       Skip the backend build step"
    echo "  --skip-frontend      Skip the frontend build step"
    echo "  --only <step>        Run only the given step (backend|frontend), plus docker"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Note: the docker step always runs."
}

# ------------------------------------------------------------
# Parse arguments
# ------------------------------------------------------------
while [[ $# -gt 0 ]]; do
    case "$1" in
        --skip-backend)
            RUN_BACKEND=false
            shift
            ;;
        --skip-frontend)
            RUN_FRONTEND=false
            shift
            ;;
        --only)
            if [[ $# -lt 2 ]]; then
                echo "Error: --only requires a value (backend|frontend)"
                exit 1
            fi
            case "$2" in
                backend)
                    RUN_BACKEND=true
                    RUN_FRONTEND=false
                    ;;
                frontend)
                    RUN_BACKEND=false
                    RUN_FRONTEND=true
                    ;;
                *)
                    echo "Error: unknown value for --only: $2"
                    echo "Valid values are: backend, frontend"
                    exit 1
                    ;;
            esac
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Error: unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# ------------------------------------------------------------
# Step: backend
# ------------------------------------------------------------
build_backend() {
    local project_path="$1"
    echo "Building backend..."
    cd "$project_path/backend" || exit 1
    cargo build --release || exit 1
}

# ------------------------------------------------------------
# Step: frontend
# ------------------------------------------------------------
build_frontend() {
    local project_path="$1"
    echo "Building frontend..."
    cd "$project_path/frontend" || exit 1
    pnpm build || exit 1
}

# ------------------------------------------------------------
# Step: docker (always runs)
# ------------------------------------------------------------
build_docker() {
    local project_path="$1"
    echo "Starting Docker..."
    cd "$project_path" || exit 1
    docker compose down
    docker compose up -d --build || exit 1
}

# ------------------------------------------------------------
# Run steps
# ------------------------------------------------------------
if [[ "$RUN_BACKEND" == true ]]; then
    build_backend "$PROJECT_PATH"
else
    echo "Skipping backend build..."
fi

if [[ "$RUN_FRONTEND" == true ]]; then
    build_frontend "$PROJECT_PATH"
else
    echo "Skipping frontend build..."
fi

build_docker "$PROJECT_PATH"

echo "Build completed!"