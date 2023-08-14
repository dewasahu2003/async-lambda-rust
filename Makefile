format:
	cargo fmt

lint:
	cargo clippy 
run:
	cargo run 

release:
	cargo lambda build --release

deploy:
	cargo lambda deploy s3l --profile admin-2 -r us-east-1 

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"name": "count"}' \
  		--output-format json \
  		s3l --profile admin-2 -r us-east-1 --verbose

all: format lint test run