.PHONY: deploy
deploy: check-env
	cd api
	cargo lambda build --release
	cargo lambda deploy \
		--profile ${AWS_PROFILE} \
		--enable-function-url \
		--binary-name krypto_api \
		krypto_api

.PHONY: check-env
check-env:
ifndef AWS_PROFILE
	$(error AWS_PROFILE is undefined)
endif
