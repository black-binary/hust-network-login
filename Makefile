all: x86_64-unknown-linux-musl mips-unknown-linux-musl mipsel-unknown-linux-musl arm-unknown-linux-musleabi x86_64-pc-windows-gnu 

mips-unknown-linux-musl:
	cross build --release --target $@
	mips-linux-gnu-strip ./target/$@/release/hust-network-login
	zip $@.zip -j ./target/$@/release/hust-network-login

mipsel-unknown-linux-musl:
	cross build --release --target $@
	mips-linux-gnu-strip ./target/$@/release/hust-network-login
	zip $@.zip -j ./target/$@/release/hust-network-login

arm-unknown-linux-musleabi:
	cross build --release --target $@
	arm-linux-gnueabihf-strip ./target/$@/release/hust-network-login
	zip $@.zip -j ./target/$@/release/hust-network-login

x86_64-pc-windows-gnu:
	cross build --release --target $@
	strip ./target/$@/release/hust-network-login.exe
	zip $@.zip -j ./target/$@/release/hust-network-login.exe

x86_64-unknown-linux-musl:
	cargo build --release --target $@
	strip ./target/$@/release/hust-network-login
	zip $@.zip -j ./target/$@/release/hust-network-login