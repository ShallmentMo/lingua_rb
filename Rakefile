# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("lingua.gemspec")

RbSys::ExtensionTask.new("lingua", GEMSPEC) do |ext|
  ext.lib_dir = "lib/lingua"
end

task default: %i[compile test rubocop]
