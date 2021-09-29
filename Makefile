.PHONY: build

build:
		@cargo build --release
		@mkdir -p netlify/functions
		@cp target/release/webhooks_twilio_rs netlify/functions

local:
		cargo build --release
		cp target/release/webhooks_twilio_rs netlify/functions
