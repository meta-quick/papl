java_run: lib
	javac HelloWorld.java && java -Djava.library.path=mylib/target/debug/ HelloWorld

.PHONY: lib

javah:
	javac -h . src/main/com/datasafe/papl/Engine.java

lib:
	cargo build