#!/usr/bin/env bash
# ------------------------------------------------------------
# Build script
#
# Usage:
#   ./build.sh backend
#   ./build.sh frontend
#   ./build.sh docker
#   ./build.sh all
#
# You can also combine steps:
#   ./build.sh backend frontend
#   ./build.sh backend docker
#   ./build.sh frontend docker
# ------------------------------------------------------------

set -euo pipefail

PROJECT_PATH="$(pwd)"

RUN_BACKEND=false
RUN_FRONTEND=false
RUN_DOCKER=false

show_help() {
    echo "Usage: ./build.sh [backend|frontend|docker|all]"
    echo ""
    echo "Steps:"
    echo "  backend    Build backend"
    echo "  frontend   Build frontend"
    echo "  docker     Build and start Docker"
    echo "  all        Build backend + frontend + Docker"
    echo ""
    echo "Examples:"
    echo "  ./build.sh backend"
    echo "  ./build.sh frontend"
    echo "  ./build.sh docker"
    echo "  ./build.sh all"
    echo "  ./build.sh backend docker"
}

# ------------------------------------------------------------
# Parse arguments
# ------------------------------------------------------------

if [[ $# -eq 0 ]]; then
    echo "Error: no build step specified."
    echo ""
    show_help
    exit 1
fi

while [[ $# -gt 0 ]]; do
    case "$1" in
        backend)
            RUN_BACKEND=true
            ;;

        frontend)
            RUN_FRONTEND=true
            ;;

        docker)
            RUN_DOCKER=true
            ;;

        all)
            RUN_BACKEND=true
            RUN_FRONTEND=true
            RUN_DOCKER=true
            ;;

        -h|--help)
            show_help
            exit 0
            ;;

        *)
            echo "Error: unknown build step: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac

    shift
done

# ------------------------------------------------------------
# Step: backend
# ------------------------------------------------------------

build_backend() {
    echo "========================================"
    echo "Building backend..."
    echo "========================================"

    cd "$PROJECT_PATH/backend"
    cargo build --release
}

# ------------------------------------------------------------
# Step: frontend
# ------------------------------------------------------------

build_frontend() {
    echo "========================================"
    echo "Building frontend..."
    echo "========================================"

    cd "$PROJECT_PATH/frontend"
    pnpm build
}

# ------------------------------------------------------------
# Step: docker
# ------------------------------------------------------------

build_docker() {
    echo "========================================"
    echo "Starting Docker..."
    echo "========================================"

    cd "$PROJECT_PATH"

    docker compose down
    docker compose up -d --build
}

# ------------------------------------------------------------
# Run selected steps
# ------------------------------------------------------------

if [[ "$RUN_BACKEND" == true ]]; then
    build_backend
fi

if [[ "$RUN_FRONTEND" == true ]]; then
    build_frontend
fi

if [[ "$RUN_DOCKER" == true ]]; then
    build_docker
fi

echo ""
echo "Build completed!"