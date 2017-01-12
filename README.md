<div id="table-of-contents">
<h2>Table of Contents</h2>
<div id="text-table-of-contents">
<ul>
<li><a href="#org20ec46c">1. Task</a>
<ul>
<li><a href="#org080c85a">1.1. What are terms?</a></li>
<li><a href="#orgc1cd6a2">1.2. Requirements</a>
<ul>
<li><a href="#org9a69b08">1.2.1. Flexibility and Genericy</a></li>
<li><a href="#org2791c6d">1.2.2. Usability</a></li>
<li><a href="#org08d81f5">1.2.3. Performance</a></li>
<li><a href="#org7857313">1.2.4. Functionality</a></li>
<li><a href="#org1e950c2">1.2.5. Stability</a></li>
</ul>
</li>
</ul>
</li>
<li><a href="#org6616517">2. How to start</a>
<ul>
<li><a href="#org51580c0">2.1. Installing Rust</a></li>
<li><a href="#org75f085b">2.2. Running the example tokenizer</a></li>
<li><a href="#orgd4ef23e">2.3. Writing and running unittests</a></li>
<li><a href="#org1537055">2.4. Resources</a></li>
</ul>
</li>
<li><a href="#org995eed9">3. Where to start</a>
<ul>
<li><a href="#org807bbe0">3.1. Get comfortable with stdin</a></li>
<li><a href="#orge4a827f">3.2. Write a function signature that takes a string and returns a vector of terms</a></li>
<li><a href="#org84d0137">3.3. Implement that function using the split<sub>whitespace</sub> Method</a></li>
<li><a href="#org4c693a0">3.4. Write unit tests for that function</a></li>
<li><a href="#org8ebe891">3.5. Implement this function as an iterator</a></li>
<li><a href="#org33fda06">3.6. Adapt your unit tests to handle iterators. (Tip: use collect)</a></li>
<li><a href="#org33a1274">3.7. Repeat for normalizers and stemmers</a></li>
</ul>
</li>
</ul>
</div>
</div>

<a id="org20ec46c"></a>

# Task

Build a tool that takes a string through stdin and prints resulting terms to stdout seperated by newline.


<a id="org080c85a"></a>

## What are terms?

Terms are the result of [tokenization](https://en.wikipedia.org/wiki/Tokenization_(lexical_analysis)), [normalization](https://en.wikipedia.org/wiki/Canonicalization) and [stemming](https://en.wikipedia.org/wiki/Stemming) a string. 
For example the sentence "This is a house" has four terms: "this" "is" "a" "house".


<a id="orgc1cd6a2"></a>

## Requirements


<a id="org9a69b08"></a>

### Flexibility and Genericy

You want to build sustainable and reusable software.
That is why you should think of a common [trait](https://doc.rust-lang.org/book/traits.html) for each tokenizers, normalizers and stemmers.
Maybe you can also think of a common trait for all three of them. Or for a wrapper structure.


<a id="org2791c6d"></a>

### Usability

You should also think of a way how to chain them.
If there are two tokenizers, three normalizers and seven stemmers the user should be able to chain them to his need.


<a id="org08d81f5"></a>

### Performance

Make these analyzers lazy! They should not do anything unless asked for results. 
You might consider using [iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) to achieve this. 


<a id="org7857313"></a>

### Functionality

For now the interfaces are more important than functionality. 
A tokenizer that splits at non-alphanumeric chars, 
a normalizer that lowercases every token and a very basic stemmer should suffice.


<a id="org1e950c2"></a>

### Stability

While using existing [crates](https://crates.io/) is ok, please make sure they are stable enough to be used in a stable product.
Please also refrain from using the nightly release channel of rust.


<a id="org6616517"></a>

# How to start

This repository contains a whitespace tokenizer in its simplest form.
You can compile it using \`cargo build\` and run it with \`cargo run\`.
But first you have to install rust:


<a id="org51580c0"></a>

## Installing Rust

See [rustup.rs](https://rustup.rs/) the official Rust installer.


<a id="org75f085b"></a>

## Running the example tokenizer

    echo "this is a house" > test.txt
    cat test.txt | cargo run


<a id="orgd4ef23e"></a>

## Writing and running unittests

The example code contains the structure of a test. 
You can execute tests by running \`cargo test\`.
Check [the Book](https://doc.rust-lang.org/book/testing.html) for more!


<a id="org1537055"></a>

## Resources

[Standard Library Reference](https://doc.rust-lang.org/std/)
[The Book](https://doc.rust-lang.org/book/)
[The Guide to Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)


<a id="org995eed9"></a>

# Where to start

I would propose the following iterations:


<a id="org807bbe0"></a>

## Get comfortable with stdin


<a id="orge4a827f"></a>

## Write a function signature that takes a string and returns a vector of terms


<a id="org84d0137"></a>

## Implement that function using the [split<sub>whitespace</sub>](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) Method


<a id="org4c693a0"></a>

## Write unit tests for that function


<a id="org8ebe891"></a>

## Implement this function as an iterator


<a id="org33fda06"></a>

## Adapt your unit tests to handle iterators. (Tip: use [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect))


<a id="org33a1274"></a>

## Repeat for normalizers and stemmers

By then you should feel comfortable enough to go on on your own!

