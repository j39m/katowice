#!/bin/bash

# `wl-paste -w` suppresses newlines by default, and there's no way to
# force them back on. Also — some quirk (of Nightly?) causes each copy
# event to fire twice, apparently?

perl -pe 's%data:image/png;base64,%\n%g' >> klaus
