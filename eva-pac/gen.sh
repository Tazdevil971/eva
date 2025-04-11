#!/bin/sh
halogen --version

STM32_DATA_REPO="https://github.com/embassy-rs/stm32-data-generated.git"
STM32_DATA_HASH="94123805f3c2fd7bfbb0f2ed8a4e2b18eb60a1bc"

(cd /tmp
    git clone ${STM32_DATA_REPO}
    cd stm32-data-generated
    git checkout ${STM32_DATA_HASH})

echo "Generating stm32 pacs..."
halogen stm32-data-convert -i /tmp/stm32-data-generated/data --filter "stm32(f4|f7).*" -o - | \
halogen gen-rust \
    -i - \
    -o src/stm32 \
    --core-path "../cortexm" \
    --utils embed \
    --format rustfmt

echo "Generating cortex-m pacs..."
halogen gen-rust \
    -i defs/cortexm/index.json \
    -o src/cortexm \
    --utils none \
    --format rustfmt \
    --dont-gen-chips \
    --dont-gen-list 