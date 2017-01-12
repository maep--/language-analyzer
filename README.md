<div id="table-of-contents">
<h2>Table of Contents</h2>
<div id="text-table-of-contents">
<ul>
<li><a href="#org1d743f7">1. Task</a>
<ul>
<li><a href="#org8b45a17">1.1. What are terms?</a></li>
<li><a href="#org216ef50">1.2. Requirements</a>
<ul>
<li><a href="#orgada4994">1.2.1. Flexibility and Genericy</a></li>
<li><a href="#orgd7c449c">1.2.2. Usability</a></li>
<li><a href="#org134472f">1.2.3. Performance</a></li>
<li><a href="#org3bbab77">1.2.4. Functionality</a></li>
<li><a href="#orgcf00447">1.2.5. Stability</a></li>
</ul>
</li>
</ul>
</li>
<li><a href="#org8dd85db">2. How to start</a>
<ul>
<li><a href="#org7ac3484">2.1. Installing Rust</a></li>
<li><a href="#org23fa835">2.2. Running the example tokenizer</a></li>
<li><a href="#orgb2d7415">2.3. Writing and running unittests</a></li>
<li><a href="#orgd6442a7">2.4. Resources</a></li>
</ul>
</li>
<li><a href="#org1f1f6ac">3. Where to start</a></li>
</ul>
</div>
</div>

<a id="org1d743f7"></a>

# Task

Build a tool that takes a string through stdin and prints resulting terms to stdout seperated by newline.


<a id="org8b45a17"></a>

## What are terms?

Terms are the result of [tokenization](https://en.wikipedia.org/wiki/Tokenization_(lexical_analysis)), [normalization](https://en.wikipedia.org/wiki/Canonicalization) and [stemming](https://en.wikipedia.org/wiki/Stemming) a string. 
For example the sentence "This is a house" has four terms: "this" "is" "a" "house".


<a id="org216ef50"></a>

## Requirements


<a id="orgada4994"></a>

### Flexibility and Genericy

You want to build sustainable and reusable software.
That is why you should think of a common [trait](https://doc.rust-lang.org/book/traits.html) for each tokenizers, normalizers and stemmers.
Maybe you can also think of a common trait for all three of them. Or for a wrapper structure.


<a id="orgd7c449c"></a>

### Usability

You should also think of a way how to chain them.
If there are two tokenizers, three normalizers and seven stemmers the user should be able to chain them to his need.


<a id="org134472f"></a>

### Performance

Make these analyzers lazy! They should not do anything unless asked for results. 
You might consider using [iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) to achieve this. 


<a id="org3bbab77"></a>

### Functionality

For now the interfaces are more important than functionality. 
A tokenizer that splits at non-alphanumeric chars, 
a normalizer that lowercases every token and a very basic stemmer should suffice.


<a id="orgcf00447"></a>

### Stability

While using existing [crates](https://crates.io/) is ok, please make sure they are stable enough to be used in a stable product.
Please also refrain from using the nightly release channel of rust.


<a id="org8dd85db"></a>

# How to start

This repository contains a whitespace tokenizer in its simplest form.
You can compile it using \`cargo build\` and run it with \`cargo run\`.
But first you have to install rust:


<a id="org7ac3484"></a>

## Installing Rust

See [rustup.rs](https://rustup.rs/) the official Rust installer.


<a id="org23fa835"></a>

## Running the example tokenizer

    echo "this is a house" > test.txt
    cat test.txt | cargo run


<a id="orgb2d7415"></a>

## Writing and running unittests

The example code contains the structure of a test. 
You can execute tests by running \`cargo test\`.
Check [the Book](https://doc.rust-lang.org/book/testing.html) for more!


<a id="orgd6442a7"></a>

## Resources

[Standard Library Reference](https://doc.rust-lang.org/std/)
[The Book](https://doc.rust-lang.org/book/)
[The Guide to Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)


<a id="org1f1f6ac"></a>

# Where to start

Fork this repo. Clone it locally.

Then I would propose the following iterations:

-   Get comfortable with stdin
-   Write a function signature that takes a string and returns a vector of terms
-   Implement that function using the [split<sub>whitespace</sub>](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) Method
-   Write unit tests for that function
-   Implement this function as an iterator
-   Adapt your unit tests to handle iterators. (Tip: use [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect))
-   Repeat for normalizers and stemmers

By then you should feel comfortable enough to go on on your own!

