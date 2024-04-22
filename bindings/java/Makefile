java_run: lib
	javac HelloWorld.java && java -Djava.library.path=mylib/target/debug/ HelloWorld

.PHONY: lib

javah:
	javac -h .  src/main/java/com/datasafe/papl/Engine.java src/main/java/com/datasafe/papl/EngineType.java

lib:
	cargo build