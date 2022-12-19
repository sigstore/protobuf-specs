import org.gradle.api.publish.maven.internal.publication.DefaultMavenPublication

plugins {
    `java-library`
    `maven-publish`
    id("com.google.protobuf") version "0.9.1"
    id("com.diffplug.spotless") version "6.11.0"
}

description = "Code generated library for the Sigstore bundle format protobufs"

sourceSets {
    main {
        proto {
            srcDir("../protos/")
        }
    }
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:3.21.9")
    implementation("com.google.api.grpc:proto-google-common-protos:2.11.0")
}

// gradle reproducible jar builds
tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

java {
    withJavadocJar()
    withSourcesJar()
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

val repoUrl = "https://github.com/sigstore/protobuf-specs"

publishing {
    publications {
        create<MavenPublication>("mavenJava") {

            artifactId = project.name
            from(components["java"])

            versionMapping {
                usage(Usage.JAVA_RUNTIME) {
                    fromResolutionResult()
                }
                usage(Usage.JAVA_API) {
                    fromResolutionOf("runtimeClasspath")
                }
            }
            pom {
                name.set(
                    (project.findProperty("artifact.name") as? String)
                        ?: project.name
                )
                description.set(
                    project.provider { project.description }
                )
                inceptionYear.set("2022")
                url.set(repoUrl)
                organization {
                    name.set("Sigstore")
                    url.set("https://sigstore.dev")
                }
                developers {
                    developer {
                        organization.set("Sigstore authors")
                        organizationUrl.set("https://sigstore.dev")
                    }
                }
                issueManagement {
                    system.set("GitHub Issues")
                    url.set("$repoUrl/issues")
                }
                licenses {
                    license {
                        name.set("Apache-2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    }
                }
                scm {
                    connection.set("scm:git:$repoUrl.git")
                    developerConnection.set("scm:git:$repoUrl.git")
                    url.set(repoUrl)
                    tag.set("HEAD")
                }
            }
        }
    }
}

// this task should be used by github actions to create release artifacts along with a slsa
// attestation.
tasks.register("createReleaseBundle") {
    val releaseDir = layout.buildDirectory.dir("release")
    outputs.dir(releaseDir)
    dependsOn((publishing.publications["mavenJava"] as DefaultMavenPublication).publishableArtifacts)
    doLast {
        project.copy {
            from((publishing.publications["mavenJava"] as DefaultMavenPublication).publishableArtifacts.files)
            into(releaseDir)
            rename("pom-default.xml", "${project.name}-${project.version}.pom")
            rename("module.json", "${project.name}-${project.version}.module")
        }
    }
}
