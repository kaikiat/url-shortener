.PHONY: docker build

docker:
	docker build --platform linux/amd64 -t url-shortener .
	docker tag url-shortener kaikiatpoh/url-shortener
	docker push kaikiatpoh/url-shortener