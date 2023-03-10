require File.expand_path("../lib/sigstore_protobuf_specs/version", __FILE__)

Gem::Specification.new do |spec|
  spec.name          = "sigstore_protobuf_specs"

  spec.version       = Dev::Sigstore::VERSION
  spec.authors       = ["Sigstore Authors"]
  spec.email         = ["sigstore-dev@googlegroups.com"]

  spec.summary       = %q{A library for serializing and deserializing Sigstore messages.}
  spec.homepage      = "https://www.sigstore.dev/"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")

  spec.license       = 'Apache-2.0'
  spec.required_ruby_version = ">= 2.7.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/sigstore/protobuf-specs"
  spec.metadata["bug_tracker_uri"] = "https://github.com/sigstore/protobuf-specs/issues"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir["README.md", "LICENSE", "lib/**/*"]
  spec.bindir        = "bin"
  spec.executables   = spec.files.grep(%r{^bin/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_runtime_dependency 'google-protobuf', '~> 3.21', '>= 3.21.12'
  spec.add_runtime_dependency 'googleapis-common-protos-types', '~> 1.4'
end
