#!/usr/bin/env bash

sea-orm-cli generate entity -o backend/src/entity/db_model/new \
	--with-copy-enums \
	--date-time-crate chrono \
	-u postgresql://financrr:password@localhost:5432/financrr
