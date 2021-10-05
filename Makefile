.PHONY: build

build:
		cd image-component && npm i && npm run build && rm -rf node_modules
		@cargo build --release
		@mkdir -p netlify/functions
		@cp target/release/webhooks_twilio netlify/functions/webhooks_twilio_rs
		@cp target/release/generate_image netlify/functions/generate_image_rs

local:
		cargo build --release
		cp target/release/webhooks_twilio netlify/functions/webhooks_twilio_rs
		cp target/release/generate_image netlify/functions/generate_image_rs
