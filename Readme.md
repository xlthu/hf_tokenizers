# C++ interface of Huggingface Tokenizers

## Usage

`cargo build --release`

Generated:
- include: `target/cxxbridge/hf_tokenizers/src/lib.rs.h`
- src: `target/cxxbridge/hf_tokenizers/src/lib.rs.cc`
- lib: `target/release/libhf_tokenizers.a`

## Interface

```c++
namespace hf {

// Load from a `tokenizer.json` to create a `Tokenizer`
// If failed, return `default_tokenizer()` with `ok() == false`
::rust::Box<::hf::Tokenizer> load_tokenizer(::rust::Str file_path) noexcept;
::rust::Box<::hf::Tokenizer> default_tokenizer() noexcept;

struct Tokenizer final : public ::rust::Opaque {
    // If this tokenizer is ready for use
    bool ok() const noexcept;

    // Encode `text`, return encoded `token ids`
    ::rust::Vec<::std::uint32_t> encode(::rust::Str text, bool add_special_tokens) const noexcept;

    // Decode `token ids`, return decoded `text`
    ::rust::String decode(::rust::Slice<::std::uint32_t const> ids, bool skip_special_tokens) const noexcept;

    // `token` <-> `id` conversion
    ::std::uint32_t token_to_id(::rust::Str token) const noexcept;
    ::rust::String id_to_token(::std::uint32_t id) const noexcept;
};

} // namespace hf
```
