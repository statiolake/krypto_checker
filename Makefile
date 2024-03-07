.PHONY: deploy
deploy: check-env
	cargo lambda build --release
	cargo lambda deploy \
		--profile ${AWS_PROFILE} \
		--enable-function-url \
		--binary-name krypto_api \
		krypto_api

# With `make watch`, you can access by for example:
# http://localhost:9000/lambda-url/krypto_api/api/v1/solve?cards=1,2,2,3&target=1
.PHONY: watch
watch:
	cargo lambda watch

.PHONY: check-env
check-env:
ifndef AWS_PROFILE
	$(error AWS_PROFILE is undefined)
endif
