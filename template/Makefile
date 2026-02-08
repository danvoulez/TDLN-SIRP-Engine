.PHONY: ci
ci:
	python3 ci/check_rref.py artifacts/card.json
	python3 ci/check_sirp.py artifacts/card.json
	python3 ci/verify_signature_stub.py artifacts/request.json artifacts/headers.txt artifacts/pubkey.b64
