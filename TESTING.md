# How to run tests

From the root level i.e. the level where this document is currently sitting, you would execute:

`bazel build //utils/{lib_directory_name}:{library_test_name}`

Where `lib_directory_name` is the name of the directory containing the `BUILD.bazel` file for the library, and `library_test_name` is the name given to the test in `BUILD.bazel`. See the `name` attribute in `rust_test` in `BUILD.bazel`.

So to execute the `greeter` library test(s) you would run:

`bazel build //utils/greeter_lib:greeter_test`

And for `reader` it would be:

`bazel build //utils/reader_lib:reader_test`

<!-- TODO: Integration test suite -->

Working on integration tests 

~~Now, for running the integration test suite~~