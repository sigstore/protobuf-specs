plugins {
    `java-library`
    id("com.google.protobuf") version "0.9.1"
    id("com.diffplug.spotless") version "6.11.0"
}

sourceSets {
    main {
        proto {
            srcDir("../protos/")
        }
    }
}

group = "dev.sigstore"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:3.21.9")
}

// gradle reproducible jar builds
tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.21.9"
    }
}

spotless {
    kotlinGradle {
        target("*.gradle.kts") // default target for kotlinGradle
        ktlint()
    }
    format("misc") {
        target("*.md", ".gitignore", "**/*.yaml")

        trimTrailingWhitespace()
        indentWithSpaces()
        endWithNewline()
    }
    // we have no non-generated java code
}
