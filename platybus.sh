#!/usr/bin/env bash

# working directory: script directory
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "${SCRIPT_DIR}"

# create output directory
mkdir -p dist

# create script file for platybus
cat >dist/script <<EOF
#!/usr/bin/env bash

exec ./renamer-rust "\$@"
EOF

chmod u+x dist/script

# call platybus
CMD=("platypus -y \
--droppable \
--name 'Renamer' \
--bundle-identifier 'org.pewpeo.renamer' \
--author 'Peter Oltmann' \
--app-version $(git describe --tags --always) \
--interface-type 'Droplet' \
--interpreter '/bin/sh' \
--uniform-type-identifiers 'public.item|public.folder' \
--bundled-file 'target/release/renamer-rust' \
'dist/script' 'dist/Renamer'")

echo "${CMD}"
eval ${CMD}
