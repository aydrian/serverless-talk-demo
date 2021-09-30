.PHONY: build

build:
		cd image-component && npm i && npm run build && rm -rf node_modules
		@cargo build --release
		@mkdir -p netlify/functions
		@cp target/release/webhooks_twilio_rs netlify/functions

local:
		cargo build --release
		cp target/release/webhooks_twilio_rs netlify/functions
