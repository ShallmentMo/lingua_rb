# Lingua

[![Gem Version](https://badge.fury.io/rb/lingua_rb.svg)](https://badge.fury.io/rb/lingua_rb)
![CI](https://github.com/ShallmentMo/lingua_rb/actions/workflows/main.yml/badge.svg)
![License](https://img.shields.io/github/license/ShallmentMo/lingua_rb)
![Gems](https://img.shields.io/gem/dt/lingua_rb)

Lingua is an [Ruby][0] wrapper for the [Rust][1] [lingua][2] crate with [magnus][3].

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add lingua_rb

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install lingua_rb

## Usage

```ruby
irb> Lingua.detect("this is definitely English")
=> "English"

irb> Lingua.detect("וזה בעברית")
=> "Hebrew"

irb> Lingua.detect("państwowych", languages: %w[english russian polish])
=> "Polish"

irb> Lingua.detect("כלב", languages: %w[english russian polish])
=> nil
```

**Note:** The value of `languages` option should be an array of String. An array of symbols will be ignored.

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ShallmentMo/lingua. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/ShallmentMo/lingua/blob/master/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the Lingua project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/[USERNAME]/lingua/blob/master/CODE_OF_CONDUCT.md).

## Acknowledgements

- [lingua_ex](https://github.com/joshrotenberg/lingua_ex)
- [magnus](https://github.com/matsadler/magnus)

[0]: https://ruby-lang.org
[1]: https://www.rust-lang.org
[2]: https://crates.io/crates/lingua
[3]: https://github.com/matsadler/magnus
