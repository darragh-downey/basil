# Shared libraries

Bazel does support [granular package visibility](https://docs.bazel.build/versions/main/be/functions.html#package), but for this demo we want these libraries to be public to the wider project so we can build for multiple different targets - otherwise what is the point of the monorepo other than a giant ball of spaghetti?
