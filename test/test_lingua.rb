# frozen_string_literal: true

require "test_helper"

class TestLingua < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Lingua::VERSION
  end

  def test_it_detect_english
    assert_equal "English", Lingua.detect("this is definitely English")
  end
end
