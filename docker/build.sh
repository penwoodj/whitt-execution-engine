#!/bin/bash
set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

LLAMA_CPP_VERSION="${LLAMA_CPP_VERSION:-b4450}"
IMAGE_NAME="${IMAGE_NAME:-whitt-execution-engine/llama-server}"
TAG="${TAG:-latest}"
PUSH="${PUSH:-false}"

while [[ $# -gt 0 ]]; do
    case $1 in
        --version) LLAMA_CPP_VERSION="$2"; shift 2 ;;
        --tag) TAG="$2"; shift 2 ;;
        --name) IMAGE_NAME="$2"; shift 2 ;;
        --push) PUSH=true; shift ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "  --version VERSION    llama.cpp version (default: b4450)"
            echo "  --tag TAG            Image tag (default: latest)"
            echo "  --name NAME          Image name"
            echo "  --push               Push image to registry"
            exit 0 ;;
        *) log_warn "Unknown option: $1"; shift ;;
    esac
done

FULL_IMAGE_NAME="${IMAGE_NAME}:${TAG}"

log_info "Building Docker image..."
log_info "  llama.cpp version: ${LLAMA_CPP_VERSION}"
log_info "  Image name: ${FULL_IMAGE_NAME}"

docker build \
    --build-arg LLAMA_CPP_VERSION="${LLAMA_CPP_VERSION}" \
    -t "${FULL_IMAGE_NAME}" \
    -f docker/Dockerfile \
    .

log_info "Build complete: ${FULL_IMAGE_NAME}"
IMAGE_SIZE=$(docker images "${FULL_IMAGE_NAME}" --format "{{.Size}}")
log_info "Image size: ${IMAGE_SIZE}"

if [ "$PUSH" = "true" ]; then
    log_info "Pushing image to registry..."
    docker push "${FULL_IMAGE_NAME}"
    log_info "Push complete"
fi
