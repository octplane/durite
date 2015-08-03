target/release/durite: src/
	docker run --rm -it -v $(CURDIR):/source schickling/rust cargo build --release
clean:
	rm -rf target
