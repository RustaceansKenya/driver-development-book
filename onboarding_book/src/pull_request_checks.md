# Pull Request Checks

Before a pull-request gets merged, it goes through some uniformity and quality checks.  
The scripts of those checks are implemented as github workflows found [here][github-workflows].   

If your commits modified the driver-tutorial only, then your pull request will undergo DOC_CHECKS that will be discussed in the next chapter.  
All Docs follow the [Rust-Doc style guide][rust-doc-style-guide].  


If your commits modified the driver-code or driver-crate-documentation, your pull-request will undergo CODEBASE_CHECKS that will be discussed in a later chapter.  


<!-- hard-link -->
[github-workflows]: https://github.com/RustaceansKenya/driver-development-book/tree/master/.github/workflows
<!-- hard-link -->
[rust-doc-style-guide]: https://github.com/esp-rs/book/blob/main/rust-doc-style-guide.md