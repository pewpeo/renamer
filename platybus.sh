#!/usr/bin/env bash

# working directory: script directory
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}"

# determine node version directory
NODE_VERSION=$(node -v)
NODE_VERSION_DIR=$(npm -g prefix)

# create output directory
mkdir -p dist

# create script file for platybus
cat > dist/script <<EOF
#!/usr/bin/env bash

exec ${NODE_VERSION}/bin/node renamer.mjs "\$@"
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
--bundled-file '"${NODE_VERSION_DIR}"' \
--bundled-file 'rename.js' \
--bundled-file 'renamer.mjs' \
'dist/script' 'dist/Renamer'")

echo "${CMD}"
eval ${CMD}
