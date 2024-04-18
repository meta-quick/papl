# PAPL Java

**PAPL** is wrapper layer for rego and cedar policies.

## Usage

Regorus Java is published with native libraries for the following:
- 64-bit Linux (kernel 3.2+, glibc 2.17+)
- ARM64 Linux (kernel 4.1, glibc 2.17+)
- 64-bit macOS (10.12+, Sierra+)
- ARM64 macOS (11.0+, Big Sur+)
- 64-bit MSVC (Windows 7+)

If you need to run it in a different OS or an architecture you need to manually [build it](#Building).

If you're on one of the supported platforms, you can just pull prebuilt JAR from Maven Central by declaring a dependency on `com.datasafe.papl:papl-binding-java`.

With Maven:
```xml
<dependencies>
    <dependency>
        <groupId>com.datasafe.papl</groupId>
        <artifactId>papl-binding-java</artifactId>
        <version>0.0.1</version>
    </dependency>
</dependencies>
```

With [Gradle](https://gradle.org/):
```kotlin
// build.gradle.kts
implementation("com.datasafe.papl:papl-binding-java:0.0.1")
```

## Building

In order to build papl Java for a target platform, you need to install Rust target
for that target platform first:

```bash
$ rustup target add aarch64-apple-darwin
```

Afterwards, you can build native library for that target using:
```bash
$ cargo build --release --target aarch64-apple-darwin
```

You will then have a native library at `../../target/aarch64-apple-darwin/release/libregorus_java.dylib` depending on your target.

You can then build a JAR from source using:
```bash
$ mvn package
```

And you will have a JAR at `./target/regorus-java-0.0.1.jar`.

You need to make sure both of the artifacts in Java's classpath.
For example with `java` CLI:
```bash
$ java -Djava.library.path=../../target/aarch64-apple-darwin/release/ -cp target/regorus-java-0.0.1.jar Test.java
```

