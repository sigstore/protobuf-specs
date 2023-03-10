# sigstore_protobuf_specs

These are the Ruby language bindings for Sigstore's protobuf specs.

See the [repository's README](https://github.com/sigstore/protobuf-specs)
for more information.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'sigstore_protobuf_specs'
```

And then execute:
```bash
    $ bundle install
```
Or install it yourself as:
```bash
    $ gem install sigstore_protobuf_specs
```

## Usage

Import all the bindings:

```ruby
require 'sigstore_protobuf_specs'
```

Or you can import them individually:

```ruby
require 'sigstore_bundle_pb'
```

See what is available in `gen/pb-ruby/lib/`.

## Releasing

Make sure you update the version in `gen/pb-ruby/lib/sigstore_protobuf_specs/version.rb`

A release will be build and automatically pushed to RubyGems when a tag in the
format `release/ruby/v*` is created.

Contact elfotografo007 for Gem ownership stuff.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/sigstore/protobuf-specs/issues.

