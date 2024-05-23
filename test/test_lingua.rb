# frozen_string_literal: true

require "test_helper"

class TestLingua < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Lingua::VERSION
  end

  def test_it_detect_english
    assert_equal "English", Lingua.detect("this is definitely English")
  end

  def test_it_detect_chinese
    assert_equal "Chinese", Lingua.detect("你好，我来自中国")
  end

  def test_it_detect_hebrew
    assert_equal "Hebrew", Lingua.detect("וזה בעברית")
  end

  def test_it_detect_polish_with_languages
    assert_equal "Polish", Lingua.detect("państwowych", { languages: %i[english russian polish] })
  end

  def test_it_detect_nil
    assert_nil Lingua.detect("כלב", { "languages" => %w[english russian polish] })
  end

  def test_it_support_minimum_relative_distance
    assert_nil Lingua.detect(
      "languages are awesome",
      { "languages" => %w[english french german spanish], "minimum_relative_distance" => 0.9 }
    )
  end

  def test_it_support_is_every_language_model_preloaded
    assert "English", Lingua.detect(
      "languages are awesome",
      { "languages" => %w[english french german spanish], "is_every_language_model_preloaded" => true }
    )
  end

  def test_it_support_is_low_accuracy_mode_enabled
    assert "English", Lingua.detect(
      "languages are awesome",
      { "languages" => %w[english french german spanish], "is_low_accuracy_mode_enabled" => true }
    )
  end
end
