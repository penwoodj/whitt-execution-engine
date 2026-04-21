#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect GPU type
detect_gpu() {
    log_info "Detecting GPU type..."

    # Check for NVIDIA
    if command -v nvidia-smi &> /dev/null; then
        log_info "NVIDIA GPU detected"
        nvidia-smi --query-gpu=name,driver_version,memory.total --format=csv,noheader

        # Check NVIDIA Container Toolkit version (CRITICAL for security)
        if command -v nvidia-container-cli &> /dev/null; then
            local nvidia_ctk_version=$(nvidia-container-cli --version 2>/dev/null | head -n1 || echo "unknown")
            log_info "NVIDIA Container Toolkit: $nvidia_ctk_version"

            # Check if version is 1.17.4+ (CVE-2025-23266/23359 fix)
            local major_minor=$(echo "$nvidia_ctk_version" | grep -oP '^\d+\.\d+' || echo "0.0")
            if [ "$(echo "$major_minor" | cut -d. -f1)" -lt 1 ] || { [ "$(echo "$major_minor" | cut -d. -f1)" -eq 1 ] && [ "$(echo "$major_minor" | cut -d. -f2)" -lt 17 ]; }; then
                log_error "NVIDIA Container Toolkit version is too old: $nvidia_ctk_version"
                log_error "Version 1.17.4+ REQUIRED for CVE-2025-23266/23359 fixes (CVSS 9.0)"
                log_error "Update: sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit"
                exit 1
            fi
        else
            log_warn "nvidia-container-cli not found. Cannot verify toolkit version."
        fi

        echo "nvidia"
        return 0
    fi

    # Check for AMD (via /dev/kfd)
    if [ -e /dev/kfd ]; then
        log_info "AMD GPU detected"

        # Check AMD Container Toolkit version (required for --gpus support)
        if command -v amdgpu-container-cli &> /dev/null; then
            local amdgpu_ctk_version=$(amdgpu-container-cli --version 2>/dev/null | head -n1 || echo "unknown")
            log_info "AMD Container Toolkit: $amdgpu_ctk_version"

            # Check if version is 1.2.0+ (required for --gpus support)
            local major_minor=$(echo "$amdgpu_ctk_version" | grep -oP '^\d+\.\d+' || echo "0.0")
            if [ "$(echo "$major_minor" | cut -d. -f1)" -lt 1 ] || { [ "$(echo "$major_minor" | cut -d. -f1)" -eq 1 ] && [ "$(echo "$major_minor" | cut -d. -f2)" -lt 2 ]; }; then
                log_warn "AMD Container Toolkit version is too old: $amdgpu_ctk_version"
                log_warn "Version 1.2.0+ required for --gpus support"
                log_warn "Update: sudo apt-get update && sudo apt-get install -y amdgpu-container-toolkit"
                log_warn "Will use legacy device passthrough (--device=/dev/dri, --device=/dev/kfd)"
            fi
        else
            log_warn "amdgpu-container-cli not found. Using legacy device passthrough."
        fi

        # Try to get GPU info (if available)
        if command -v rocminfo &> /dev/null; then
            rocminfo | grep "Name:" | head -n1
        else
            echo "AMD GPU (details unavailable)"
        fi
        echo "amd"
        return 0
    fi

    # Check for AMD (via lspci)
    if command -v lspci &> /dev/null; then
        if lspci | grep -i "AMD" | grep -i "VGA" > /dev/null; then
            log_info "AMD GPU detected (via lspci)"
            lspci | grep -i "AMD" | grep -i "VGA"
            echo "amd"
            return 0
        fi
    fi

    log_warn "No GPU detected. Running in CPU-only mode."
    echo "cpu"
    return 0
}

# Main
GPU_TYPE=$(detect_gpu)

log_info "Recommended compose file:"
case "$GPU_TYPE" in
    nvidia)
        echo "  docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d"
        ;;
    amd)
        echo "  docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d"
        ;;
    cpu)
        echo "  docker compose up -d"
        ;;
esac
