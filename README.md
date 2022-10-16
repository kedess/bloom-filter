## bloom-filter

The Bloom filter is a probabilistic data structure that allows you to check whether an element belongs to a set. At the same time, it is possible to get a false positive response (there is no element in the set, but the data structure reports that it exists), but not a false negative.
A frequent application of the bloom filter is caching database requests. First, let's check that the existence of the element in the filter and if it is definitely not there, then you do not need to go to the database, otherwise you need to look at the existence of the element in the database.
Compared to hash tables, the Bloom filter can manage several orders of magnitude smaller amounts of memory, sacrificing determinism.

### Usage example::
```rust
use bloom_filter::BloomFilter;

fn main() {
    let n = 8 * 1024 * 1024; // data buffer
    let m = 1000000; // кnumber of elements in the filter
    let k = 2; // number of hash functions
    let mut bloom_filter = BloomFilter::build(n, m, k);

    println!("{:.4}", bloom_filter.false_positive_probability());

    bloom_filter.insert("google");
    bloom_filter.insert("facebook");
    bloom_filter.insert("yandex");

    assert_eq!(bloom_filter.contains("google"), true);
    assert_eq!(bloom_filter.contains("facebook"), true);
    assert_eq!(bloom_filter.contains("yandex"), true);
    assert_eq!(bloom_filter.contains("microsoft"), false);
    assert_eq!(bloom_filter.contains("oracle"), false);
    assert_eq!(bloom_filter.contains("redhat"), false);
}
```

### Cargo.toml
```bash
[dependencies]
radix-trie = {git = "https://github.com/mingendo/bloom-filter.git", branch="main"}
```
