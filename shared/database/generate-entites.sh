#!/bin/sh
sea-orm-cli generate entity --with-serde both --with-copy-enums --date-time-crate chrono --expanded-format -o src/orm