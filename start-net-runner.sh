#!/bin/bash

rm -rf /tmp/minimal-runtime-test
mkdir -p /tmp/minimal-runtime-test
${OASIS_CORE_PATH}/bin/oasis-net-runner \
    --fixture.default.node.binary ${OASIS_CORE_PATH}/bin/oasis-node \
    --fixture.default.runtime.binary target/debug/minimal-runtime \
    --fixture.default.runtime.loader ${OASIS_CORE_PATH}/bin/oasis-core-runtime-loader \
    --fixture.default.runtime.provisioner unconfined \
    --fixture.default.keymanager.binary '' \
    --basedir /tmp/minimal-runtime-test \
    --basedir.no_temp_dir
